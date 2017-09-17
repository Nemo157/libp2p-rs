use std::io;
use std::rc::Rc;

use maddr::MultiAddr;
use multistream::Negotiator;
use identity::{ HostId, PeerId };
use tokio_core::reactor;
use futures::{ future, Future, Poll };
use tokio_io::codec::FramedParts;

use mplex;

use transport::Transport;
use mux::Stream;
use muxmux::MultiplexerSquared;
use { PeerInfo };

struct State {
    muxmux: MultiplexerSquared,
}

pub struct Peer { state: Rc<State> }

impl Peer {
    pub(crate) fn new(host: HostId, info: PeerInfo, event_loop: reactor::Handle) -> Peer {
        println!("New peer {:?}", info);
        let muxmux = MultiplexerSquared::new(host, info, event_loop);
        Peer { state: Rc::new(State { muxmux }) }
    }

    pub(crate) fn start_accept(host: HostId, conn: Transport, addr: MultiAddr) -> impl Future<Item=(PeerId, Stream, MultiAddr), Error=io::Error> {
        MultiplexerSquared::start_accept(host, conn)
            .map(move |(id, conn)| (id, conn, addr))
    }

    pub(crate) fn finish_accept(&mut self, conn: Stream, addr: MultiAddr) {
        self.state.muxmux.finish_accept(conn, addr);
    }

    pub fn id(&self) -> PeerId {
        self.state.muxmux.id()
    }

    pub fn open_stream(&mut self, protocol: &'static [u8]) -> impl Future<Item=FramedParts<mplex::Stream>, Error=io::Error> {
        self.state.muxmux.new_stream()
            .and_then(move |stream| Negotiator::start(stream, true)
                .negotiate(protocol, move |parts: FramedParts<mplex::Stream>| -> Box<Future<Item=_,Error=_>> {
                    Box::new(future::ok(parts))
                })
                .finish())
    }

    pub fn poll_accept(&mut self) -> Poll<Option<mplex::Stream>, io::Error> {
        self.state.muxmux.poll_accept()
    }
}

impl Clone for Peer {
    fn clone(&self) -> Self {
        Peer { state: self.state.clone() }
    }
}
