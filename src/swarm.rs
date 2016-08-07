use std::io;
use identity::HostId;

use { PeerInfo, Transport };
use peer::Peer;

#[derive(Debug)]
pub struct Swarm {
    id: HostId,
    allow_unencrypted: bool,
    peers: Vec<Peer>,
    transports: Vec<Box<Transport>>,
}

impl Swarm {
    pub fn new(id: HostId, allow_unencrypted: bool) -> Swarm {
        Swarm {
            id: id,
            allow_unencrypted: allow_unencrypted,
            peers: Vec::new(),
            transports: Vec::new()
        }
    }

    pub fn add_transport<T: 'static>(&mut self, transport: T) where T: Transport {
        self.transports.push(Box::new(transport));
    }

    pub fn add_transports<I, T>(&mut self, transports: T)
        where
            I: Iterator<Item=Box<Transport>>,
            T: IntoIterator<Item=Box<Transport>, IntoIter=I>
    {
        self.transports.extend(transports.into_iter());
    }

    pub fn add_peer(&mut self, info: PeerInfo) {
        self.peers.push(Peer::new(info, self.allow_unencrypted));
    }

    pub fn add_peers<I, T>(&mut self, peers: T)
        where
            I: Iterator<Item=PeerInfo>,
            T: IntoIterator<Item=PeerInfo, IntoIter=I>
    {
        let allow_unencrypted = self.allow_unencrypted;
        self.peers.extend(peers.into_iter().map(|info| Peer::new(info, allow_unencrypted)));
    }

    pub fn pre_connect_all(&mut self) -> Vec<io::Result<()>> {
        let id = &self.id;
        let transports = &mut self.transports;
        self.peers.iter_mut().map(|peer| peer.pre_connect(id, transports)).collect()
    }
}
