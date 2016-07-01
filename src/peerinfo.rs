use multiaddr::MultiAddr;
use peerid::PeerId;

pub struct PeerInfo {
    id: PeerId,
    addresses: Vec<MultiAddr>,
}

impl PeerInfo {
    pub fn new(id: PeerId) -> PeerInfo {
        PeerInfo { id: id, addresses: Vec::new() }
    }
}
