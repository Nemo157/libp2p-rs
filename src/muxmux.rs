use std::cell::{Ref, RefCell};
use std::fmt;
use std::io;
use std::rc::Rc;

use slog::{b, log, kv, record, record_static};
use slog::{error, info, warn, o, Logger};
use maddr::MultiAddr;
use identity::{ HostId, PeerId };
use tokio_core::reactor;
use futures::{ Async, Future, Poll };

use mplex;

use PeerInfo;
use transport::Transport;
use mux::{ Stream, EventuallyMultiplexer };

pub(crate) struct MultiplexerSquared {
    logger: Logger,
    host: HostId,
    info: PeerInfo,
    event_loop: reactor::Handle,
    muxes: Rc<RefCell<Vec<EventuallyMultiplexer>>>,
}

impl MultiplexerSquared {
    pub(crate) fn new(logger: Logger, host: HostId, info: PeerInfo, event_loop: reactor::Handle) -> MultiplexerSquared {
        let muxes = Rc::new(RefCell::new(Vec::new()));
        MultiplexerSquared { logger, host, info, event_loop, muxes }
    }

    pub(crate) fn start_accept(logger: Logger, host: HostId, conn: Transport) -> impl Future<Item=(PeerId, Stream), Error=io::Error> {
        EventuallyMultiplexer::start_accept(logger, host, conn)
    }

    pub(crate) fn finish_accept(&self, conn: Stream, addr: MultiAddr) {
        let logger = self.logger.new(o!("addr" => addr.to_string()));
        info!(logger, "muxmux::finish_accept");
        let mux = EventuallyMultiplexer::finish_accept(logger, conn);
        self.muxes.borrow_mut().push(mux);
    }

    pub(crate) fn id(&self) -> PeerId {
        self.info.id.clone()
    }

    pub(crate) fn poll_accept(&self) -> Poll<Option<mplex::Stream>, io::Error> {
        let mut muxes = self.muxes.borrow_mut();
        let mut i = 0;
        while i < muxes.len() {
            match muxes[i].poll_accept() {
                Ok(Async::Ready(Some(conn))) => {
                    return Ok(Async::Ready(Some(conn)));
                }
                Ok(Async::Ready(None)) => {
                    info!(self.logger, "Multiplexer {:?} to closed", muxes[i]);
                    muxes.swap_remove(i);
                }
                Ok(Async::NotReady) => {
                    i += 1;
                }
                Err(err) => {
                    error!(self.logger, "Error on multiplexer {:?}: {:?}", muxes[i], err);
                    muxes.swap_remove(i);
                }
            }
        }
        return Ok(Async::NotReady);
    }

    fn choose_mux(&self) -> Ref<EventuallyMultiplexer> {
        if { self.muxes.borrow().is_empty() } {
            warn!(self.logger, "No existing mux");
            let mux = EventuallyMultiplexer::connect(self.logger.clone(), self.host.clone(), self.info.clone(), self.event_loop.clone());
            self.muxes.borrow_mut().push(mux);
        }
        Ref::map(self.muxes.borrow(), |muxes| muxes.iter().next().unwrap())
    }

    pub(crate) fn new_stream(&self) -> impl Future<Item=mplex::Stream, Error=io::Error> {
        self.choose_mux().new_stream()
    }
}

impl fmt::Debug for MultiplexerSquared {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            f.debug_struct("MultiplexerSquared")
                .field("muxes", &self.muxes.borrow())
                .finish()
        } else {
            f.debug_struct("MultiplexerSquared")
                .field("host", &self.host)
                .field("info", &self.info)
                .field("event_loop", &self.event_loop)
                .field("muxes", &self.muxes)
                .finish()
        }
    }
}
