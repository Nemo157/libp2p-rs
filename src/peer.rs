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

use msgio;

use { PeerInfo, transport };

trait Transport: Stream<Item=Vec<u8>, Error=io::Error> + Sink<SinkItem=Vec<u8>, SinkError=io::Error> {
}

impl<S> Transport for S where S: Stream<Item=Vec<u8>, Error=io::Error> + Sink<SinkItem=Vec<u8>, SinkError=io::Error> {
}

struct State {
    host: HostId,
    info: PeerInfo,
    allow_unencrypted: bool,
    event_loop: reactor::Handle,
    idle_connection: RefCell<Option<SecStream<Framed<transport::Transport, msgio::Codec>>>>,
}

enum PreConnectFuture {
    Connecting {
        state: Rc<State>,
        attempt: Box<Future<Item=SecStream<Framed<transport::Transport, msgio::Codec>>, Error=io::Error>>,
        addrs: Vec<MultiAddr>,
    },
    Done,
}

pub struct Peer(Rc<State>);

impl Clone for Peer {
    fn clone(&self) -> Self { Peer(self.0.clone()) }
}

impl Peer {
    pub fn new(host: HostId, info: PeerInfo, allow_unencrypted: bool, event_loop: reactor::Handle) -> Peer {
        Peer(Rc::new(State {
            host: host,
            info: info,
            allow_unencrypted: allow_unencrypted,
            event_loop: event_loop.clone(),
            idle_connection: RefCell::new(None),
        }))
    }

    pub fn pre_connect(&mut self) -> impl Future<Item=(), Error=()> {
        State::pre_connect(self.0.clone())
    }
}

impl State {
    fn pre_connect(state: Rc<Self>) -> PreConnectFuture {
        if state.idle_connection.borrow().is_some() {
            println!("Peer {:?} already connected", state.info.id());
            PreConnectFuture::Done
        } else {
            let mut addrs = Vec::from_iter(state.info.addrs().iter().cloned());
            if let Some(addr) = addrs.pop() {
                let attempt = state.connect(&addr);
                PreConnectFuture::Connecting {
                    state: state,
                    attempt: attempt,
                    addrs: addrs,
                }
            } else {
                PreConnectFuture::Done
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

impl Future for PreConnectFuture {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match *self {
            PreConnectFuture::Connecting { ref state, ref mut attempt, ref mut addrs } => {
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
