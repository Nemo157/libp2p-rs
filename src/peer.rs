use std::io;
use std::iter::FromIterator;
use std::rc::Rc;
use std::cell::RefCell;

use maddr::MultiAddr;
use multistream::Negotiator;
use secio::{ self, SecStream };
use identity::{ HostId, PeerId };
use tokio_core::reactor;
use futures::{ future, Future, Poll };
use tokio_core::io::{ Io, Framed };

use msgio;
use mplex;

use { PeerInfo, transport };

struct State {
    host: HostId,
    info: PeerInfo,
    allow_unencrypted: bool,
    event_loop: reactor::Handle,
    idle_connection: RefCell<Option<SecStream<Framed<transport::Transport, msgio::Codec>>>>,
    mux: RefCell<Option<mplex::Multiplexer<SecStream<Framed<transport::Transport, msgio::Codec>>>>>,
}

struct ConnectFuture {
    state: Rc<State>,
    attempt: Box<Future<Item=SecStream<Framed<transport::Transport, msgio::Codec>>, Error=io::Error>>,
    addrs: Vec<MultiAddr>,
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
            mux: RefCell::new(None),
        }))
    }

    pub fn id(&self) -> &PeerId {
        self.0.info.id()
    }

    pub fn pre_connect(&mut self) -> impl Future<Item=(), Error=()> {
        State::pre_connect(self.0.clone())
    }

    pub fn open_stream(&mut self, protocol: &str) -> impl Future<Item=mplex::Stream, Error=io::Error> {
        State::open_stream(self.0.clone(), protocol)
    }
}

impl State {
    fn pre_connect(state: Rc<Self>) -> impl Future<Item=(), Error=()> {
        // TODO: Needed to avoid linker errors
        fn log(e: io::Error) { println!("{:?}", e); }
        if state.idle_connection.borrow().is_some() {
            println!("Peer {:?} already connected", state.info.id());
            future::Either::A(future::ok(()))
        } else {
            future::Either::B(State::do_connect(state.clone()).map(move |conn| {
                *state.idle_connection.borrow_mut() = Some(conn);
            }).map_err(log))
        }
    }

    fn do_connect(state: Rc<Self>) -> impl Future<Item=SecStream<Framed<transport::Transport, msgio::Codec>>, Error=io::Error> {
        let mut addrs = Vec::from_iter(state.info.addrs().iter().cloned());
        if let Some(addr) = addrs.pop() {
            let attempt = state.connect(&addr);
            future::Either::A(ConnectFuture {
                state: state,
                attempt: attempt,
                addrs: addrs,
            })
        } else {
            future::Either::B(future::err(io::Error::new(io::ErrorKind::Other, "No addresses")))
        }
    }

    fn connect_stream(state: Rc<Self>) -> impl Future<Item=SecStream<Framed<transport::Transport, msgio::Codec>>, Error=io::Error> {
        if let Some(connection) = state.idle_connection.borrow_mut().take() {
            println!("Peer {:?} already connected", state.info.id());
            return future::Either::A(future::ok(connection));
        }
        future::Either::B(State::do_connect(state))
    }

    fn connect_mux(state: Rc<Self>) -> impl Future<Item=mplex::Multiplexer<SecStream<Framed<transport::Transport, msgio::Codec>>>, Error=io::Error> {
        State::connect_stream(state)
            .and_then(|conn| {
                Negotiator::start(conn)
                    .negotiate(b"/mplex/6.7.0", move |conn: SecStream<Framed<transport::Transport, msgio::Codec>>| -> Box<Future<Item=_, Error=_>> {
                        Box::new(future::ok(mplex::Multiplexer::new(conn, true)))
                    })
                    .finish()
            })
    }

    fn ensure_mux(state: Rc<Self>) -> impl Future<Item=Rc<Self>, Error=io::Error> {
        if state.mux.borrow().is_some() {
            println!("Peer {:?} already muxed", state.info.id());
            future::Either::A(future::ok(state))
        } else {
            println!("Peer {:?} needs muxing", state.info.id());
            future::Either::B(State::connect_mux(state.clone()).map(move |mux| {
                *state.mux.borrow_mut() = Some(mux);
                state
            }))
        }
    }

    fn open_stream(state: Rc<Self>, protocol: &str) -> impl Future<Item=mplex::Stream, Error=io::Error> {
        State::ensure_mux(state)
            .and_then(|state| {
                if let Some(ref mut mux) = *state.mux.borrow_mut() {
                    mux.new_stream()
                    // TODO: protocol?
                } else {
                    // TODO: Needed to avoid linker errors
                    // panic!("TODO: Should not get here");
                    ::std::process::abort()
                }
            })
    }

    fn connect(&self, addr: &MultiAddr) -> Box<Future<Item=SecStream<Framed<transport::Transport, msgio::Codec>>, Error=io::Error>> {
        let host = self.host.clone();
        let peer_id = self.info.id().clone();
        println!("Connecting peer {:?} via {}", peer_id, addr);
        Box::new(transport::connect(&addr, &self.event_loop)
            .and_then(move |conn| {
                let negotiator = Negotiator::start(conn.framed(msgio::Codec(msgio::Prefix::VarInt, msgio::Suffix::NewLine)))
                    .negotiate(b"/secio/1.0.0", move |framed: Framed<transport::Transport, msgio::Codec>| -> Box<Future<Item=_,Error=_>> {
                        Box::new(secio::handshake(framed.into_inner().framed(msgio::Codec(msgio::Prefix::BigEndianU32, msgio::Suffix::None)), host, peer_id))
                    });
                // if allow_unencrypted {
                //     negotiator = negotiator.negotiate(b"/plaintext/1.0.0", |conn| -> Box<Future<Item=Box<Transport>, Error=io::Error>> { Box::new(future::ok(Box::new(conn.framed(msgio::Prefix::BigEndianU32, msgio::Suffix::None)) as Box<Transport>)) });
                // }
                negotiator.finish()
            }))
    }
}

impl Future for ConnectFuture {
    type Item = SecStream<Framed<transport::Transport, msgio::Codec>>;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.attempt.poll() {
            Ok(result) => Ok(result),
            Err(err) => {
                println!("Failed to connect: {:?}", err);
                if let Some(addr) = self.addrs.pop() {
                    self.attempt = self.state.connect(&addr);
                    self.poll()
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Failed to connect to all addresses"))
                }
            }
        }
    }
}
