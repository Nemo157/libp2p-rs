use std::io;
use std::iter::FromIterator;
use std::rc::Rc;
use std::cell::{Ref, RefCell};

use maddr::MultiAddr;
use multistream::Negotiator;
use secio::{ self, SecStream };
use identity::{ HostId, PeerId };
use tokio_core::reactor;
use futures::{ future, task, Async, Future, Poll, Stream };
use futures::task::Task;
use tokio_io::codec::FramedParts;

use mplex;

use { PeerInfo, transport };

struct State {
    host: HostId,
    info: PeerInfo,
    allow_unencrypted: bool,
    event_loop: reactor::Handle,
    idle_connection: RefCell<Option<SecStream<transport::Transport>>>,
    mux: RefCell<Option<mplex::Multiplexer<SecStream<transport::Transport>>>>,
    task: RefCell<Option<Task>>,
}

struct ConnectFuture {
    state: Rc<State>,
    attempt: Box<Future<Item=(PeerId, SecStream<transport::Transport>), Error=io::Error>>,
    addrs: Vec<MultiAddr>,
}

pub struct Peer(Rc<State>);

impl Clone for Peer {
    fn clone(&self) -> Self { Peer(self.0.clone()) }
}

impl Peer {
    pub fn new(host: HostId, info: PeerInfo, allow_unencrypted: bool, event_loop: reactor::Handle) -> Peer {
        Peer(Rc::new(State {
            host,
            info,
            allow_unencrypted,
            event_loop: event_loop.clone(),
            idle_connection: RefCell::new(None),
            mux: RefCell::new(None),
            task: RefCell::new(None),
        }))
    }

    pub fn id(&self) -> Ref<PeerId> {
        self.0.info.id()
    }

    pub fn pre_connect(&mut self) -> impl Future<Item=(), Error=()> {
        State::pre_connect(self.0.clone())
    }

    pub fn open_stream(&mut self, protocol: &'static [u8]) -> impl Future<Item=FramedParts<mplex::Stream>, Error=io::Error> {
        State::open_stream(self.0.clone(), protocol)
    }
}

impl Stream for Peer {
    type Item = mplex::Stream;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let mut my_task = self.0.task.borrow_mut();
        if my_task.is_none() {
            *my_task = Some(task::current());
        }
        if let Some(ref mut mux) = *self.0.mux.borrow_mut() {
            if let Async::Ready(Some(stream)) = mux.poll()? {
                println!("New incoming stream {:?} for peer {:?}", stream, self.0.info);
                return Ok(Async::Ready(Some(stream)));
            }
        }
        Ok(Async::NotReady)
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

    fn do_connect(state: Rc<Self>) -> impl Future<Item=SecStream<transport::Transport>, Error=io::Error> {
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

    fn connect_stream(state: Rc<Self>) -> impl Future<Item=SecStream<transport::Transport>, Error=io::Error> {
        if let Some(connection) = state.idle_connection.borrow_mut().take() {
            println!("Peer {:?} already connected", state.info.id());
            return future::Either::A(future::ok(connection));
        }
        future::Either::B(State::do_connect(state))
    }

    fn connect_mux(state: Rc<Self>) -> impl Future<Item=mplex::Multiplexer<SecStream<transport::Transport>>, Error=io::Error> {
        State::connect_stream(state)
            .and_then(|conn| {
                Negotiator::start(conn, true)
                    .negotiate(b"/mplex/6.7.0", move |parts: FramedParts<SecStream<transport::Transport>>| -> Box<Future<Item=_, Error=_>> {
                        Box::new(future::ok(mplex::Multiplexer::from_parts(parts, true)))
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
                if let Some(ref task) = *state.task.borrow() {
                    task.notify();
                }
                state
            }))
        }
    }

    fn open_stream(state: Rc<Self>, protocol: &'static [u8]) -> impl Future<Item=FramedParts<mplex::Stream>, Error=io::Error> {
        State::ensure_mux(state)
            .and_then(move |state| {
                if let Some(ref mut mux) = *state.mux.borrow_mut() {
                    mux.new_stream()
                        .and_then(move |stream| {
                            Negotiator::start(stream, true)
                                .negotiate(protocol, move |parts: FramedParts<mplex::Stream>| -> Box<Future<Item=_,Error=_> + 'static> {
                                    Box::new(future::ok(parts))
                                })
                                .finish()
                        })
                } else {
                    // TODO: Needed to avoid linker errors
                    // panic!("TODO: Should not get here");
                    ::std::process::abort()
                }
            })
    }

    fn connect(&self, addr: &MultiAddr) -> Box<Future<Item=(PeerId, SecStream<transport::Transport>), Error=io::Error>> {
        let host = self.host.clone();
        let peer_id = self.info.id().clone();
        println!("Connecting peer {:?} via {}", peer_id, addr);
        Box::new(transport::connect(&addr, &self.event_loop)
            .and_then(move |conn| {
                let negotiator = Negotiator::start(conn, true)
                    .negotiate(b"/secio/1.0.0", move |parts: FramedParts<transport::Transport>| -> Box<Future<Item=_,Error=_>> {
                        Box::new(secio::handshake(parts, host, peer_id))
                    });
                // if allow_unencrypted {
                //     negotiator = negotiator.negotiate(b"/plaintext/1.0.0", |conn| -> Box<Future<Item=Box<Transport>, Error=io::Error>> { Box::new(future::ok(Box::new(conn.framed(msgio::Prefix::BigEndianU32, msgio::Suffix::None)) as Box<Transport>)) });
                // }
                negotiator.finish()
            }))
    }
}

impl Future for ConnectFuture {
    type Item = SecStream<transport::Transport>;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.attempt.poll() {
            Ok(Async::Ready((id, stream))) => {
                if !self.state.info.id().proven() {
                    self.state.info.update_id(id);
                }
                Ok(Async::Ready(stream))
            }
            Ok(Async::NotReady) => {
                Ok(Async::NotReady)
            }
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
