use futures::{ future, Future, Stream, Sink };
use futures::sync::mpsc;
use tokio_core::reactor;
use identity::HostId;

use { PeerInfo };
use peer::Peer;

enum Msg {
    AddPeer(PeerInfo),
    AddPeers(Vec<PeerInfo>),
    PreConnectAll,
}

#[derive(Clone)]
pub struct Swarm {
    sender: mpsc::Sender<Msg>,
}

impl Swarm {
    pub fn new(id: HostId, allow_unencrypted: bool, event_loop: reactor::Handle) -> Swarm {
        let (sender, receiver) = mpsc::channel(1);
        let mut peers = Vec::new();
        event_loop.clone().spawn(receiver.and_then(move |msg| match msg {
            Msg::AddPeer(info) => {
                println!("Adding peer {:?}", info);
                peers.push(Peer::new(id.clone(), info, allow_unencrypted, event_loop.clone()));
                Box::new(future::ok(())) as Box<Future<Item=(), Error=()>>
            }
            Msg::AddPeers(infos) => {
                println!("Adding peers {:?}", infos);
                peers.extend(infos.into_iter().map(|info| Peer::new(id.clone(), info, allow_unencrypted, event_loop.clone())));
                Box::new(future::ok(())) as Box<Future<Item=(), Error=()>>
            }
            Msg::PreConnectAll => {
                println!("Pre connecting peers");
                Box::new(future::join_all(peers.iter_mut().map(|peer| peer.pre_connect()).collect::<Vec<_>>())
                    .map(|_| ())) as Box<Future<Item=(), Error=()>>
            }
        }).for_each(|_| Ok(())));
        Swarm { sender: sender }
    }

    fn send(&mut self, msg: Msg) -> impl Future<Item=(), Error=()> {
        self.sender.clone()
            .send(msg)
            .map(|_| ())
            .map_err(|_| ())
    }

    pub fn add_peer(&mut self, info: PeerInfo) -> impl Future<Item=(), Error=()> {
        self.send(Msg::AddPeer(info))
    }

    pub fn add_peers(&mut self, infos: Vec<PeerInfo>) -> impl Future<Item=(), Error=()> {
        self.send(Msg::AddPeers(infos))
    }

    pub fn pre_connect_all(&mut self) -> impl Future<Item=(), Error=()> {
        self.send(Msg::PreConnectAll)
    }
}
