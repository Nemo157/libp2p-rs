use std::io;
use std::rc::Rc;
use std::cell::{Ref, RefCell};

use maddr::MultiAddr;
use identity::{ HostId, PeerId };
use tokio_core::reactor;
use futures::{ Async, Future, Poll };

use mplex;

use PeerInfo;
use transport::Transport;
use mux::{ Stream, EventuallyMultiplexer };

pub(crate) struct MultiplexerSquared {
    host: HostId,
    info: PeerInfo,
    event_loop: reactor::Handle,
    muxes: Rc<RefCell<Vec<EventuallyMultiplexer>>>,
}

impl MultiplexerSquared {
    pub(crate) fn new(host: HostId, info: PeerInfo, event_loop: reactor::Handle) -> MultiplexerSquared {
        let muxes = Rc::new(RefCell::new(Vec::new()));
        MultiplexerSquared { host, info, event_loop, muxes }
    }

    pub(crate) fn start_accept(host: HostId, conn: Transport) -> impl Future<Item=(PeerId, Stream), Error=io::Error> {
        EventuallyMultiplexer::start_accept(host, conn)
    }

    pub(crate) fn finish_accept(&self, conn: Stream, addr: MultiAddr) {
        println!("muxmux::finish_accept on {} for {:?}", addr, self.info.id);
        let mux = EventuallyMultiplexer::finish_accept(conn);
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
                    println!("Multiplexer {:?} to peer {:?} closed", muxes[i], self.info.id);
                    muxes.swap_remove(i);
                }
                Ok(Async::NotReady) => {
                    i += 1;
                }
                Err(err) => {
                    println!("Error on multiplexer {:?} to peer {:?}: {:?}", muxes[i], self.info.id, err);
                    muxes.swap_remove(i);
                }
            }
        }
        return Ok(Async::NotReady);
    }

    fn choose_mux(&self) -> Ref<EventuallyMultiplexer> {
        if { self.muxes.borrow().is_empty() } {
            println!("No existing mux to {:?}", self.info);
            let mux = EventuallyMultiplexer::connect(self.host.clone(), self.info.clone(), self.event_loop.clone());
            self.muxes.borrow_mut().push(mux);
        }
        Ref::map(self.muxes.borrow(), |muxes| muxes.iter().next().unwrap())
    }

    pub(crate) fn new_stream(&self) -> impl Future<Item=mplex::Stream, Error=io::Error> {
        self.choose_mux().new_stream()
    }
}
