use std::rc::Rc;
use std::cell::RefCell;

use futures::{ future, Future };
use tokio_core::reactor;
use identity::HostId;

use { PeerInfo };
use peer::Peer;

struct State {
    id: HostId,
    allow_unencrypted: bool,
    event_loop: reactor::Handle,
    peers: RefCell<Vec<Peer>>,
}

pub struct Swarm(Rc<State>);

impl Clone for Swarm {
    fn clone(&self) -> Self { Swarm(self.0.clone()) }
}

impl Swarm {
    pub fn new(id: HostId, allow_unencrypted: bool, event_loop: reactor::Handle) -> Swarm {
        Swarm(Rc::new(State {
            id: id,
            allow_unencrypted: allow_unencrypted,
            event_loop: event_loop.clone(),
            peers: RefCell::new(Vec::new()),
        }))
    }

    pub fn add_peer(&mut self, info: PeerInfo) -> impl Future<Item=(), Error=()> {
        println!("Adding peer {:?}", info);
        let peer = Peer::new(self.0.id.clone(), info, self.0.allow_unencrypted, self.0.event_loop.clone());
        self.0.peers.borrow_mut().push(peer);
        future::ok(())
    }

    pub fn add_peers(&mut self, infos: Vec<PeerInfo>) -> impl Future<Item=(), Error=()> {
        println!("Adding peers {:?}", infos);
        let id = self.0.id.clone();
        let allow_unencrypted = self.0.allow_unencrypted;
        let event_loop = self.0.event_loop.clone();
        let peers = infos.into_iter().map(|info| Peer::new(id.clone(), info, allow_unencrypted, event_loop.clone()));
        self.0.peers.borrow_mut().extend(peers);
        future::ok(())
    }

    pub fn pre_connect_all(&mut self) -> impl Future<Item=(), Error=()> {
        println!("Pre connecting peers");
        fn discard(_: Vec<()>) { }
        future::join_all(self.0.peers.borrow_mut().iter_mut().map(|peer| peer.pre_connect()).collect::<Vec<_>>())
            .map(discard as fn(Vec<()>) -> ())
    }
}
