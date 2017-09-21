use std::fmt;
use std::io;
use std::rc::Rc;

use maddr::MultiAddr;
use multistream::Negotiator;
use identity::{ HostId, PeerId };
use tokio_core::reactor;
use futures::{ future, Future, Poll };
use tokio_io::codec::FramedParts;
use futures::prelude::{async, await};

use mplex;

use transport::Transport;
use mux::Stream;
use muxmux::MultiplexerSquared;
use { PeerInfo };

#[derive(Debug)]
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

    #[async]
    pub(crate) fn start_accept(host: HostId, conn: Transport, addr: MultiAddr) -> impl Future<Item=(PeerId, Stream, MultiAddr), Error=io::Error> {
        let (id, conn) = await!(MultiplexerSquared::start_accept(host, conn))?;
        Ok((id, conn, addr))
    }

    pub(crate) fn finish_accept(&mut self, conn: Stream, addr: MultiAddr) {
        self.state.muxmux.finish_accept(conn, addr);
    }

    pub fn id(&self) -> PeerId {
        self.state.muxmux.id()
    }

    #[async]
    pub fn open_stream<P: AsRef<str>>(self, protocol: P) -> impl Future<Item=FramedParts<mplex::Stream>, Error=io::Error> {
        let stream = await!(self.state.muxmux.new_stream())?;
        let negotiator = Negotiator::start(stream, true)
            .negotiate(protocol, move |parts: FramedParts<mplex::Stream>| -> Box<Future<Item=_,Error=_>> {
                Box::new(future::ok(parts))
            });
        Ok({ await!(negotiator.finish())? })
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

impl fmt::Debug for Peer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            f.debug_struct("Peer")
                .field("id", &self.state.muxmux.id())
                .field("muxmux", &self.state.muxmux)
                .finish()
        } else {
            f.debug_struct("Peer")
                .field("state", &self.state)
                .finish()
        }
    }
}
