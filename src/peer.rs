use std::fmt;
use std::io;
use std::rc::Rc;

use slog::{b, log, kv, record, record_static};
use slog::{error, info, o, Logger};
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
    logger: Logger,
    muxmux: MultiplexerSquared,
}

pub struct Peer { state: Rc<State> }

impl Peer {
    pub(crate) fn new(logger: Logger, host: HostId, info: PeerInfo, event_loop: reactor::Handle) -> Peer {
        let logger = logger.new(o!("peer" => format!("{:#?}", info.id())));
        info!(logger, "New peer {:?}", info);
        let muxmux = MultiplexerSquared::new(logger.clone(), host, info, event_loop);
        Peer { state: Rc::new(State { logger, muxmux }) }
    }

    pub(crate) fn start_accept(logger: Logger, host: HostId, conn: Transport, addr: MultiAddr) -> impl Future<Item=(PeerId, Stream, MultiAddr), Error=io::Error> {
        MultiplexerSquared::start_accept(logger, host, conn)
            .map(move |(id, conn)| (id, conn, addr))
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
            .negotiate(protocol, move |parts| {
                parts
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
