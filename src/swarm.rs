use peer::Peer;

use { PeerInfo, Transport };

#[derive(Debug)]
pub struct Swarm {
    peers: Vec<Peer>,
    transports: Vec<Box<Transport>>,
}

impl Swarm {
    pub fn new() -> Swarm {
        Swarm { peers: Vec::new(), transports: Vec::new() }
    }

    pub fn add_transport(&mut self, transport: Box<Transport>) {
        self.transports.push(transport);
    }

    pub fn add_transports<I, T>(&mut self, transports: T)
        where
            I: Iterator<Item=Box<Transport>>,
            T: IntoIterator<Item=Box<Transport>, IntoIter=I>
    {
        self.transports.extend(transports.into_iter());
    }

    pub fn add_peer(&mut self, info: PeerInfo) {
        self.peers.push(Peer::new(info));
    }

    pub fn add_peers<I, T>(&mut self, peers: T)
        where
            I: Iterator<Item=PeerInfo>,
            T: IntoIterator<Item=PeerInfo, IntoIter=I>
    {
        self.peers.extend(peers.into_iter().map(Peer::new));
    }

    pub fn connect_all(&mut self) -> Vec<Result<(), ()>> {
        let transports = &mut self.transports;
        self.peers.iter_mut().map(|peer| peer.connect(transports)).collect()
    }
}
