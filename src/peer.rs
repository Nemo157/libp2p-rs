use std::io;
use std::iter::FromIterator;
use std::rc::Rc;
use std::cell::RefCell;

use maddr::MultiAddr;
use multistream::Negotiator;
use secio::{ self, SecStream };
use identity::HostId;
use tokio_core::reactor;
use futures::{ Future, Stream, Sink, Async, Poll };
use tokio_core::io::{ Io, Framed };
use tokio_city_actors as actors;

use msgio;

use { PeerInfo, transport };

trait Transport: Stream<Item=Vec<u8>, Error=io::Error> + Sink<SinkItem=Vec<u8>, SinkError=io::Error> {
}

impl<S> Transport for S where S: Stream<Item=Vec<u8>, Error=io::Error> + Sink<SinkItem=Vec<u8>, SinkError=io::Error> {
}

struct PreConnect;

struct State {
    host: HostId,
    info: PeerInfo,
    allow_unencrypted: bool,
    event_loop: reactor::Handle,
    idle_connection: RefCell<Option<SecStream<Framed<transport::Transport, msgio::Codec>>>>,
}

pub enum PreConnectFuture {
    Connecting(Rc<State>, Box<Future<Item=SecStream<Framed<transport::Transport, msgio::Codec>>, Error=io::Error>>, Vec<MultiAddr>),
    Done,
}

#[derive(Clone)]
pub struct Peer {
    handle: actors::Handle<State>,
}

impl Peer {
    pub fn new(host: HostId, info: PeerInfo, allow_unencrypted: bool, event_loop: reactor::Handle) -> Peer {
        let handle = actors::spawn(State {
            host: host,
            info: info,
            allow_unencrypted: allow_unencrypted,
            event_loop: event_loop.clone(),
            idle_connection: RefCell::new(None),
        });
        Peer { handle: handle }
    }

    pub fn pre_connect(&mut self) -> PreConnectFuture {
        self.handle.run(PreConnect)
    }
}

impl State {
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

impl actors::Operation for PreConnect {
    type State = State;
    type IntoFuture = PreConnectFuture;

    fn apply(self, state: Rc<State>) -> Self::IntoFuture {
        if state.idle_connection.borrow().is_some() {
            println!("Peer {:?} already connected", state.info.id());
            PreConnectFuture::Done
        } else {
            let mut addrs = Vec::from_iter(state.info.addrs().iter().cloned());
            if let Some(addr) = addrs.pop() {
                let attempt = state.connect(&addr);
                PreConnectFuture::Connecting(state, attempt, addrs)
            } else {
                PreConnectFuture::Done
            }
        }
    }
}

impl Future for PreConnectFuture {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match *self {
            PreConnectFuture::Connecting(ref state, ref mut attempt, ref mut addrs) => {
                match attempt.poll() {
                    Ok(Async::Ready(conn)) => {
                        *state.idle_connection.borrow_mut() = Some(conn);
                        return Ok(Async::Ready(()));
                    }
                    Ok(Async::NotReady) => {
                        return Ok(Async::NotReady);
                    }
                    Err(err) => {
                        println!("Failed to connect: {:?}", err);
                        if let Some(addr) = addrs.pop() {
                            *attempt = state.connect(&addr);
                        } else {
                            println!("Failed to connect to all addresses");
                            return Ok(Async::Ready(()));
                        }
                    }
                }
            }
            PreConnectFuture::Done => {
                return Ok(Async::Ready(()));
            }
        }
        self.poll()
    }
}
