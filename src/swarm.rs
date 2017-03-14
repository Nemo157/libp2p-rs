use std::mem;

use futures::{ future, Future, Async, Poll };
use tokio_core::reactor;
use identity::HostId;
use tokio_city_actors::{ run_actor, Actor, ActorHandle, ActorCallError };

use { PeerInfo };
use peer::{ Peer, PreConnectResult };

enum Request {
    AddPeer(PeerInfo),
    AddPeers(Vec<PeerInfo>),
    PreConnectAll,
}

struct SwarmActor {
    id: HostId,
    allow_unencrypted: bool,
    event_loop: reactor::Handle,
    peers: Vec<Peer>,
}

enum SwarmActorFuture {
    Connecting(SwarmActor, future::JoinAll<Vec<PreConnectResult>>),
    Done(SwarmActor),
    Errored,
}

#[derive(Clone)]
pub struct Swarm {
    handle: ActorHandle<SwarmActor>,
}

impl Swarm {
    pub fn new(id: HostId, allow_unencrypted: bool, event_loop: reactor::Handle) -> Swarm {
        let actor = SwarmActor {
            id: id,
            allow_unencrypted: allow_unencrypted,
            event_loop: event_loop.clone(),
            peers: Vec::new(),
        };
        Swarm { handle: run_actor(&event_loop, actor) }
    }

    fn send(&mut self, req: Request) -> impl Future<Item=(), Error=()> {
        fn log(err: ActorCallError<SwarmActor>) { println!("err: {:?}", err) }
        self.handle.call(req).map_err(log)
    }

    pub fn add_peer(&mut self, info: PeerInfo) -> impl Future<Item=(), Error=()> {
        self.send(Request::AddPeer(info))
    }

    pub fn add_peers(&mut self, infos: Vec<PeerInfo>) -> impl Future<Item=(), Error=()> {
        self.send(Request::AddPeers(infos))
    }

    pub fn pre_connect_all(&mut self) -> impl Future<Item=(), Error=()> {
        self.send(Request::PreConnectAll)
    }
}

impl Actor for SwarmActor {
    type Request = Request;
    type Response = ();
    type Error = ();
    type IntoFuture = SwarmActorFuture;

    fn call(mut self, req: Self::Request) -> Self::IntoFuture {
        match req {
            Request::AddPeer(info) => {
                println!("Adding peer {:?}", info);
                self.peers.push(Peer::new(self.id.clone(), info, self.allow_unencrypted, self.event_loop.clone()));
                SwarmActorFuture::Done(self)
            }
            Request::AddPeers(infos) => {
                println!("Adding peers {:?}", infos);
                let id = self.id.clone();
                let allow_unencrypted = self.allow_unencrypted;
                let event_loop = self.event_loop.clone();
                self.peers.extend(infos.into_iter().map(|info| Peer::new(id.clone(), info, allow_unencrypted, event_loop.clone())));
                SwarmActorFuture::Done(self)
            }
            Request::PreConnectAll => {
                println!("Pre connecting peers");
                let waiting = future::join_all(self.peers.iter_mut().map(|peer| peer.pre_connect()).collect::<Vec<_>>());
                SwarmActorFuture::Connecting(self, waiting)
            }
        }
    }
}

impl Future for SwarmActorFuture {
    type Item = (SwarmActor, ());
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match mem::replace(self, SwarmActorFuture::Errored) {
            SwarmActorFuture::Connecting(swarm, mut waiting) => {
                match waiting.poll() {
                    Ok(Async::Ready(_)) => {
                        Ok(Async::Ready((swarm, ())))
                    }
                    Ok(Async::NotReady) => {
                        *self = SwarmActorFuture::Connecting(swarm, waiting);
                        Ok(Async::NotReady)
                    }
                    Err(err) => {
                        println!("Failed to connect: {:?}", err);
                        Ok(Async::Ready((swarm, ())))
                    }
                }
            }
            SwarmActorFuture::Errored => {
                Err(())
            }
            SwarmActorFuture::Done(state) => {
                Ok(Async::Ready((state, ())))
            }
        }
    }
}
