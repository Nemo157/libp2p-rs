use std::io;
use std::rc::Rc;
use std::cell::Ref;

use maddr::MultiAddr;
use multistream::Negotiator;
use identity::{ HostId, PeerId };
use tokio_core::reactor;
use futures::{ future, Future, Poll };
use tokio_io::codec::FramedParts;

use mplex;

use mux::EventuallyMultiplexer;
use { PeerInfo, transport };

struct State {
    info: PeerInfo,
    mux: EventuallyMultiplexer,
}

pub struct Peer { state: Rc<State> }

impl Peer {
    pub fn connect(host: HostId, info: PeerInfo, allow_unencrypted: bool, event_loop: reactor::Handle) -> Peer {
        println!("Connecting peer {:?}", info);
        let mux = EventuallyMultiplexer::connect(host, info.clone(), allow_unencrypted, event_loop);
        Peer { state: Rc::new(State { info, mux }) }
    }

    pub fn accept(host: HostId, conn: transport::Transport, addr: MultiAddr, allow_unencrypted: bool) -> Peer {
        let info = PeerInfo::new(PeerId::Unknown, vec![addr]);
        let mux = EventuallyMultiplexer::accept(host, info.clone(), allow_unencrypted, conn);
        Peer { state: Rc::new(State { info, mux }) }
    }

    pub fn id(&self) -> Ref<PeerId> {
        self.state.info.id()
    }

    pub fn open_stream(&mut self, protocol: &'static [u8]) -> impl Future<Item=FramedParts<mplex::Stream>, Error=io::Error> {
        self.state.mux.new_stream()
            .and_then(move |stream| Negotiator::start(stream, true)
                .negotiate(protocol, move |parts: FramedParts<mplex::Stream>| -> Box<Future<Item=_,Error=_> + 'static> {
                    Box::new(future::ok(parts))
                })
                .finish())
    }

    pub fn poll_accept(&mut self) -> Poll<Option<mplex::Stream>, io::Error> {
        self.state.mux.poll_accept()
    }
}

impl Clone for Peer {
    fn clone(&self) -> Self {
        Peer { state: self.state.clone() }
    }
}
