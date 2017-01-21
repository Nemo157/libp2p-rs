use std::{ io, mem };
use std::iter::FromIterator;

use maddr::MultiAddr;
use multistream::Negotiator;
use secio::{ self, SecStream };
use identity::HostId;
use tokio_core::reactor;
use futures::{ Future, Stream, Sink, Async, Poll };
use futures::sync::mpsc;
use tokio_core::io::{ Io, Framed };

use msgio;

use { PeerInfo, transport };

trait Transport: Stream<Item=Vec<u8>, Error=io::Error> + Sink<SinkItem=Vec<u8>, SinkError=io::Error> {
}

impl<S> Transport for S where S: Stream<Item=Vec<u8>, Error=io::Error> + Sink<SinkItem=Vec<u8>, SinkError=io::Error> {
}

enum Msg {
    PreConnect,
}

enum PeerState {
    Waiting,
    Connecting(Box<Future<Item=SecStream<Framed<transport::Transport, msgio::Codec>>, Error=io::Error>>, Vec<MultiAddr>),
    Errored,
}

struct PeerLoop {
    state: PeerState,
    host: HostId,
    info: PeerInfo,
    allow_unencrypted: bool,
    event_loop: reactor::Handle,
    receiver: mpsc::Receiver<Msg>,
    idle_connection: Option<SecStream<Framed<transport::Transport, msgio::Codec>>>,
}

#[derive(Clone)]
pub struct Peer {
    sender: mpsc::Sender<Msg>,
}

impl Peer {
    pub fn new(host: HostId, info: PeerInfo, allow_unencrypted: bool, event_loop: reactor::Handle) -> Peer {
        let (sender, receiver) = mpsc::channel(1);
        event_loop.spawn(PeerLoop {
            state: PeerState::Waiting,
            host: host,
            info: info,
            allow_unencrypted: allow_unencrypted,
            event_loop: event_loop.clone(),
            receiver: receiver,
            idle_connection: None,
        });
        Peer { sender: sender }
    }

    fn send(&mut self, msg: Msg) -> impl Future<Item=(), Error=()> {
        self.sender.clone()
            .send(msg)
            .map(|_| ())
            .map_err(|err| { println!("error: {:?}", err); () })
    }

    pub fn pre_connect(&mut self) -> impl Future<Item=(), Error=()> {
        self.send(Msg::PreConnect)
    }
}

impl PeerLoop {
    fn handle(&mut self, msg: Msg) -> PeerState {
        match msg {
            Msg::PreConnect => {
                if self.idle_connection.is_some() {
                    println!("Peer {:?} already connected", self.info.id());
                    PeerState::Waiting
                } else {
                    let mut addrs = Vec::from_iter(self.info.addrs().iter().cloned());
                    if let Some(addr) = addrs.pop() {
                        let attempt = self.connect(&addr);
                        PeerState::Connecting(attempt, addrs)
                    } else {
                        PeerState::Waiting
                    }
                }
            }
        }
    }

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

impl Future for PeerLoop {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match mem::replace(&mut self.state, PeerState::Errored) {
            PeerState::Waiting => {
                match self.receiver.poll() {
                    Ok(Async::Ready(Some(msg))) => {
                        self.state = self.handle(msg);
                        self.poll()
                    }
                    Ok(Async::Ready(None)) => {
                        self.state = PeerState::Waiting;
                        Ok(Async::Ready(()))
                    }
                    Ok(Async::NotReady) => {
                        self.state = PeerState::Waiting;
                        Ok(Async::NotReady)
                    }
                    Err(err) => {
                        println!("error: {:?}", err);
                        Err(())
                    }
                }
            }
            PeerState::Connecting(mut attempt, mut addrs) => {
                match attempt.poll() {
                    Ok(Async::Ready(conn)) => {
                        self.idle_connection = Some(conn);
                        self.state = PeerState::Waiting;
                        self.poll()
                    }
                    Ok(Async::NotReady) => {
                        self.state = PeerState::Connecting(attempt, addrs);
                        Ok(Async::NotReady)
                    }
                    Err(err) => {
                        println!("error: {:?}", err);
                        if let Some(addr) = addrs.pop() {
                            self.state = PeerState::Connecting(self.connect(&addr), addrs);
                        } else {
                            println!("Failed to connect to all addresses");
                            self.state = PeerState::Waiting;
                        }
                        self.poll()
                    }
                }
            }
            PeerState::Errored => {
                Err(())
            }
        }
    }
}
