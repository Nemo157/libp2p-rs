use std::rc::Rc;
use std::cell::RefCell;

use futures::{ future, Future };
use tokio_core::reactor;
use identity::HostId;
use tokio_city_actors as actors;

use { PeerInfo };
use peer::{ Peer, PreConnectFuture };

struct AddPeer(PeerInfo);
struct AddPeers(Vec<PeerInfo>);
struct PreConnectAll;

struct State {
    id: HostId,
    allow_unencrypted: bool,
    event_loop: reactor::Handle,
    peers: RefCell<Vec<Peer>>,
}

#[derive(Clone)]
pub struct Swarm {
    handle: actors::Handle<State>,
}

impl Swarm {
    pub fn new(id: HostId, allow_unencrypted: bool, event_loop: reactor::Handle) -> Swarm {
        let state = State {
            id: id,
            allow_unencrypted: allow_unencrypted,
            event_loop: event_loop.clone(),
            peers: RefCell::new(Vec::new()),
        };
        Swarm { handle: actors::spawn(state) }
    }

    pub fn add_peer(&mut self, info: PeerInfo) -> impl Future<Item=(), Error=()> {
        self.handle.run(AddPeer(info))
    }

    pub fn add_peers(&mut self, infos: Vec<PeerInfo>) -> impl Future<Item=(), Error=()> {
        self.handle.run(AddPeers(infos))
    }

    pub fn pre_connect_all(&mut self) -> impl Future<Item=(), Error=()> {
        self.handle.run(PreConnectAll)
    }
}

impl actors::Operation for AddPeer {
    type State = State;
    type IntoFuture = Result<(), ()>;

    fn apply(self, state: Rc<Self::State>) -> Self::IntoFuture {
        let AddPeer(info) = self;
        println!("Adding peer {:?}", info);
        let peer = Peer::new(state.id.clone(), info, state.allow_unencrypted, state.event_loop.clone());
        state.peers.borrow_mut().push(peer);
        Ok(())
    }
}

impl actors::Operation for AddPeers {
    type State = State;
    type IntoFuture = Result<(), ()>;

    fn apply(self, state: Rc<Self::State>) -> Self::IntoFuture {
        let AddPeers(infos) = self;
        println!("Adding peers {:?}", infos);
        let id = state.id.clone();
        let allow_unencrypted = state.allow_unencrypted;
        let event_loop = state.event_loop.clone();
        let peers = infos.into_iter().map(|info| Peer::new(id.clone(), info, allow_unencrypted, event_loop.clone()));
        state.peers.borrow_mut().extend(peers);
        Ok(())
    }
}

impl actors::Operation for PreConnectAll {
    type State = State;
    type IntoFuture = future::Map<future::JoinAll<Vec<PreConnectFuture>>, fn(Vec<()>)>;

    fn apply(self, state: Rc<Self::State>) -> Self::IntoFuture {
        println!("Pre connecting peers");
        fn discard(_: Vec<()>) { }
        future::join_all(state.peers.borrow_mut().iter_mut().map(|peer| peer.pre_connect()).collect::<Vec<_>>())
            .map(discard as _)
    }
}
