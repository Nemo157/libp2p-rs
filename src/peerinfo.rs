use maddr::{ MultiAddr, Segment };
use identity::PeerId;

#[derive(Clone, Debug)]
pub struct PeerInfo {
    pub(crate) id: PeerId,
    pub(crate) addrs: Vec<MultiAddr>,
}

impl PeerInfo {
    pub fn new(id: PeerId, addrs: Vec<MultiAddr>) -> PeerInfo {
        PeerInfo { id, addrs }
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
                addrs: vec![addr],
            })
        } else {
            Err(())
        }
    }

    pub fn addrs(&self) -> &[MultiAddr] {
        &self.addrs
    }

    pub fn id(&self) -> &PeerId {
        &self.id
    }
}
