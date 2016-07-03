use multiaddr::{ MultiAddr, Segment };
use peerid::PeerId;

#[derive(Debug)]
pub struct PeerInfo {
    id: PeerId,
    addresses: Vec<MultiAddr>,
}

impl PeerInfo {
    pub fn new(id: PeerId) -> PeerInfo {
        PeerInfo { id: id, addresses: Vec::new() }
    }

    /// addr should consist of `/<routing info>/ipfs/<peer id hash>`, e.g.
    /// `/ip4/104.131.131.82/tcp/4001/ipfs/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ`
    /// will create a `PeerInfo` with expected base-58 encoded MultiHash
    /// `QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ`
    /// and initial address of
    /// `/ip4/104.131.131.82/tcp/4001`
    pub fn from_addr(addr: MultiAddr) -> Result<PeerInfo, ()> {
        if let Some((addr, Segment::Ipfs(hash))) = addr.split_off_last() {
            Ok(PeerInfo {
                id: PeerId::from_hash(hash),
                addresses: vec![addr],
            })
        } else {
            Err(())
        }
    }

    pub fn addrs(&self) -> &[MultiAddr] {
        &self.addresses
    }
}
