use std::{io, mem};
use std::iter::FromIterator;
use std::rc::Rc;
use std::cell::RefCell;

use multistream::Negotiator;
use secio::{ self, SecStream };
use identity::{ HostId, PeerId };
use tokio_core::reactor;
use futures::{ future, stream, Async, Future, Poll, Stream as S };
use futures::unsync::oneshot;
use tokio_io::codec::FramedParts;

use mplex;

use PeerInfo;
use transport::{self, Transport};

type Stream = SecStream<Transport>;
type Mux = mplex::Multiplexer<Stream>;

enum State {
    Connecting(Box<Future<Item=Mux, Error=io::Error>>, Vec<oneshot::Sender<()>>),
    Connected(Mux),
    Invalid,
}

pub struct EventuallyMultiplexer {
    inner: Rc<RefCell<State>>,
}

fn negotiate_stream(conn: Transport, host: HostId, peer: PeerId, _allow_unencrypted: bool) -> impl Future<Item=Stream, Error=io::Error> {
    println!("Connected transport, negotiating stream to {:?}", peer);
    Negotiator::start(conn, true)
        .negotiate(b"/secio/1.0.0", move |parts: FramedParts<Transport>| -> Box<Future<Item=_,Error=_>> {
            Box::new(secio::handshake(parts, host, peer))
        })
        .finish()
        .map(|(_, stream)| stream)
}

fn negotiate_mux(stream: Stream) -> impl Future<Item=Mux, Error=io::Error> {
    println!("Connected stream, negotiating mux");
    Negotiator::start(stream, true)
        .negotiate(b"/mplex/6.7.0", |parts: FramedParts<SecStream<Transport>>| -> Box<Future<Item=_, Error=_>> {
            Box::new(future::ok(mplex::Multiplexer::from_parts(parts, true)))
        })
        .finish()
}

impl EventuallyMultiplexer {
    pub fn connect(host: HostId, info: PeerInfo, allow_unencrypted: bool, event_loop: reactor::Handle) -> EventuallyMultiplexer {
        println!("Connecting mux for {:?}", info);
        let addrs = Vec::from_iter(info.addrs().iter().cloned());
        let peer = info.id().clone();
        let mux = stream::iter(addrs.into_iter().map(Ok))
            .and_then(move |addr| transport::connect(&addr, &event_loop))
            .and_then(move |conn| negotiate_stream(conn, host.clone(), peer.clone(), allow_unencrypted))
            .and_then(negotiate_mux)
            .then(|res| {
                match res {
                    Ok(mux) => Ok(Some(mux)),
                    Err(err) => {
                        println!("Error connecting to peer: {:?}", err);
                        Ok(None)
                    }
                }
            })
            .filter_map(|mux| mux)
            .into_future()
            .map_err(|(err, _): (io::Error, _)| err)
            .and_then(|(mux, _)| {
                match mux {
                    Some(mux) => Ok(mux),
                    None => Err(io::Error::new(io::ErrorKind::Other, "Could not connect to any peer addresses"))
                }
            });
        EventuallyMultiplexer {
            inner: Rc::new(RefCell::new(State::Connecting(Box::new(mux), Vec::new())))
        }
    }

    pub fn accept(host: HostId, info: PeerInfo, allow_unencrypted: bool, conn: transport::Transport) -> EventuallyMultiplexer {
        let peer = info.id().clone();
        let mux = negotiate_stream(conn, host, peer, allow_unencrypted).and_then(negotiate_mux);
        EventuallyMultiplexer {
            inner: Rc::new(RefCell::new(State::Connecting(Box::new(mux), Vec::new())))
        }
    }

    pub fn poll_accept(&self) -> Poll<Option<mplex::Stream>, io::Error> {
        self.inner.borrow_mut().poll_accept()
    }

    pub fn new_stream(&self) -> impl Future<Item=mplex::Stream, Error=io::Error> {
        let inner = self.inner.clone();
        self.inner.borrow_mut()
            .await_mux()
            .and_then(move |()| {
                let mux = match *inner.borrow_mut() {
                    State::Connected(ref mut mux) => {
                        mux.new_stream()
                    }
                    _ => {
                        // TODO: Needed to avoid linker errors
                        // panic!("TODO: Should not get here");
                        ::std::process::abort()
                    }
                };
                mux
            })
    }
}

impl State {
    fn await_mux(&mut self) -> impl Future<Item=(), Error=io::Error> {
        match *self {
            State::Connecting(_, ref mut awaiting) => {
                let (sender, receiver) = oneshot::channel();
                awaiting.push(sender);
                future::Either::A(receiver.map_err(|_| io::Error::new(io::ErrorKind::Other, "mux was cancelled")))
            }
            State::Connected(_) => {
                future::Either::B(future::ok(()))
            }
            State::Invalid => {
                panic!("Invalid EventuallyMultiplexer");
            }
        }
    }

    fn poll_accept(&mut self) -> Poll<Option<mplex::Stream>, io::Error> {
        loop {
            match mem::replace(self, State::Invalid) {
                State::Connecting(mut attempt, awaiting) => {
                    match attempt.poll()? {
                        Async::Ready(mux) => {
                            for sender in awaiting {
                                let _ = sender.send(());
                            }
                            *self = State::Connected(mux);
                        }
                        Async::NotReady => {
                            *self = State::Connecting(attempt, awaiting);
                            return Ok(Async::NotReady);
                        }
                    }
                }
                State::Connected(mut mux) => {
                    let res = mux.poll();
                    if let Ok(Async::Ready(Some(ref stream))) = res {
                        println!("New incoming muxed stream {:?} for peer", stream);
                    }
                    *self = State::Connected(mux);
                    return res;
                }
                State::Invalid => {
                    panic!("Invalid EventuallyMultiplexer");
                }
            }
        }
    }
}
