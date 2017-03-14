use std::{ io, mem };
use std::iter::FromIterator;

use maddr::MultiAddr;
use multistream::Negotiator;
use secio::{ self, SecStream };
use identity::HostId;
use tokio_core::reactor;
use futures::{ Future, Stream, Sink, Async, Poll };
use tokio_core::io::{ Io, Framed };
use tokio_city_actors::{ run_actor, Actor, ActorHandle, ActorCallResult };

use msgio;

use { PeerInfo, transport };

trait Transport: Stream<Item=Vec<u8>, Error=io::Error> + Sink<SinkItem=Vec<u8>, SinkError=io::Error> {
}

impl<S> Transport for S where S: Stream<Item=Vec<u8>, Error=io::Error> + Sink<SinkItem=Vec<u8>, SinkError=io::Error> {
}

#[derive(Clone)]
enum Request {
    PreConnect,
}

struct PeerActor {
    host: HostId,
    info: PeerInfo,
    allow_unencrypted: bool,
    event_loop: reactor::Handle,
    idle_connection: Option<SecStream<Framed<transport::Transport, msgio::Codec>>>,
}

enum PeerActorFuture {
    Connecting(PeerActor, Box<Future<Item=SecStream<Framed<transport::Transport, msgio::Codec>>, Error=io::Error>>, Vec<MultiAddr>),
    Done(PeerActor),
    Errored,
}

#[derive(Clone)]
pub struct Peer {
    handle: ActorHandle<PeerActor>,
}

pub struct PreConnectResult {
    inner: ActorCallResult<PeerActor>,
}

impl Peer {
    pub fn new(host: HostId, info: PeerInfo, allow_unencrypted: bool, event_loop: reactor::Handle) -> Peer {
        let handle = run_actor(&event_loop, PeerActor {
            host: host,
            info: info,
            allow_unencrypted: allow_unencrypted,
            event_loop: event_loop.clone(),
            idle_connection: None,
        });
        Peer { handle: handle }
    }

    pub fn pre_connect(&mut self) -> PreConnectResult {
        PreConnectResult { inner: self.handle.call(Request::PreConnect) }
    }
}

impl PeerActor {
    fn connect(&self, addr: &MultiAddr) -> Box<Future<Item=SecStream<Framed<transport::Transport, msgio::Codec>>, Error=io::Error>> {
        let host = self.host.clone();
        let peer_id = self.info.id().clone();
        println!("Connecting peer {:?} via {}", peer_id, addr);
        Box::new(transport::connect(&addr, &self.event_loop)
            .and_then(move |conn| {
                let negotiator = Negotiator::start(conn)
                    .negotiate(b"/secio/1.0.0", move |conn: transport::Transport| -> Box<Future<Item=_,Error=_>> {
                        Box::new(secio::handshake(conn.framed(msgio::Codec(msgio::Prefix::BigEndianU32, msgio::Suffix::None)), host, peer_id))
                    });
                // if allow_unencrypted {
                //     negotiator = negotiator.negotiate(b"/plaintext/1.0.0", |conn| -> Box<Future<Item=Box<Transport>, Error=io::Error>> { Box::new(future::ok(Box::new(conn.framed(msgio::Prefix::BigEndianU32, msgio::Suffix::None)) as Box<Transport>)) });
                // }
                negotiator.finish()
            }))
    }
}

impl Actor for PeerActor {
    type Request = Request;
    type Response = ();
    type Error = ();
    type IntoFuture = PeerActorFuture;

    fn call(self, req: Self::Request) -> Self::IntoFuture {
        match req {
            Request::PreConnect => {
                if self.idle_connection.is_some() {
                    println!("Peer {:?} already connected", self.info.id());
                    PeerActorFuture::Done(self)
                } else {
                    let mut addrs = Vec::from_iter(self.info.addrs().iter().cloned());
                    if let Some(addr) = addrs.pop() {
                        let attempt = self.connect(&addr);
                        PeerActorFuture::Connecting(self, attempt, addrs)
                    } else {
                        PeerActorFuture::Done(self)
                    }
                }
            }
        }
    }
}

impl Future for PeerActorFuture {
    type Item = (PeerActor, ());
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match mem::replace(self, PeerActorFuture::Errored) {
            PeerActorFuture::Connecting(mut state, mut attempt, mut addrs) => {
                match attempt.poll() {
                    Ok(Async::Ready(conn)) => {
                        state.idle_connection = Some(conn);
                        Ok(Async::Ready((state, ())))
                    }
                    Ok(Async::NotReady) => {
                        *self = PeerActorFuture::Connecting(state, attempt, addrs);
                        Ok(Async::NotReady)
                    }
                    Err(err) => {
                        println!("Failed to connect: {:?}", err);
                        if let Some(addr) = addrs.pop() {
                            let attempt = state.connect(&addr);
                            *self = PeerActorFuture::Connecting(state, attempt, addrs);
                            self.poll()
                        } else {
                            println!("Failed to connect to all addresses");
                            Ok(Async::Ready((state, ())))
                        }
                    }
                }
            }
            PeerActorFuture::Errored => {
                Err(())
            }
            PeerActorFuture::Done(state) => {
                Ok(Async::Ready((state, ())))
            }
        }
    }
}

impl Future for PreConnectResult {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll().map_err(|err| println!("err: {:?}", err))
    }
}
