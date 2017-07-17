use std::io;
use std::rc::Rc;
use std::cell::RefCell;

use futures::{ future, Future, Poll, Async, Stream };
use tokio_core::reactor;
use identity::{ HostId, PeerId };
use mplex;
use multistream::Negotiator;
use tokio_io::codec::FramedParts;

use { PeerInfo };
use peer::Peer;

struct State {
    id: HostId,
    allow_unencrypted: bool,
    event_loop: reactor::Handle,
    peers: RefCell<Vec<Peer>>,
    accepting: RefCell<Vec<Box<Future<Item=(), Error=io::Error> + 'static>>>,
}

pub struct Swarm(Rc<State>);

impl Clone for Swarm {
    fn clone(&self) -> Self { Swarm(self.0.clone()) }
}

fn accept_stream(stream: mplex::Stream, peer: &Peer) -> impl Future<Item=(), Error=io::Error> {
    // TODO: Have some services to negotiate
    Negotiator::start(stream, false)
        .finish()
}

impl Swarm {
    pub fn new(id: HostId, allow_unencrypted: bool, event_loop: reactor::Handle) -> Swarm {
        Swarm(Rc::new(State {
            id: id,
            allow_unencrypted: allow_unencrypted,
            event_loop: event_loop.clone(),
            peers: RefCell::new(Vec::new()),
            accepting: RefCell::new(Vec::new()),
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

    pub fn open_stream(&mut self, id: PeerId, protocol: &'static [u8]) -> impl Future<Item=FramedParts<mplex::Stream>, Error=io::Error> {
        if let Some(peer) = self.0.peers.borrow_mut().iter_mut().find(|peer| id.matches(&*peer.id())) {
            future::Either::A(peer.open_stream(protocol))
        } else {
            future::Either::B(future::err(io::Error::new(io::ErrorKind::Other, format!("Could not find peer {:?}", id))))
        }
    }
}

impl Future for Swarm {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut accepting = self.0.accepting.borrow_mut();

        for mut peer in self.0.peers.borrow().clone() {
            while let Async::Ready(Some(stream)) = peer.poll()? {
                accepting.push(Box::new(accept_stream(stream, &peer)));
            }
        }

        let mut i = 0;
        while i < accepting.len() {
            match accepting[i].poll() {
                Ok(Async::Ready(())) => {
                    accepting.swap_remove(i);
                }
                Ok(Async::NotReady) => {
                    i += 1;
                }
                Err(err) => {
                    println!("Error while accepting peers stream: {:?}", err);
                    accepting.swap_remove(i);
                }
            }
        }

        Ok(Async::NotReady)
    }
}
