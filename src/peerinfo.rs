use std::cell::{Ref, RefCell};

use maddr::{ MultiAddr, Segment };
use identity::PeerId;

#[derive(Debug)]
pub struct PeerInfo {
    id: RefCell<PeerId>,
    addresses: Vec<MultiAddr>,
}

impl PeerInfo {
    pub fn new(id: PeerId, addresses: Vec<MultiAddr>) -> PeerInfo {
        PeerInfo { id: RefCell::new(id), addresses }
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
                id: RefCell::new(PeerId::from_hash(hash)),
                addresses: vec![addr],
            })
        } else {
            Err(())
        }
    }

    pub fn addrs(&self) -> &[MultiAddr] {
        &self.addresses
    }

    pub fn id(&self) -> Ref<PeerId> {
        self.id.borrow()
    }

    /// Allow updating this peer's id with a proven id once we obtain their
    /// public key
    // TODO: Should store public keys somewhere centralized once we acquire them
    pub fn update_id(&self, id: PeerId) {
        if let PeerId::Unknown = id { /* ok */ } else {
            if !self.id().matches(&id) {
                panic!("Attempted to update peer info with different id, expected proven id for {:?} got {:?}", self.id, id);
            }
        }
        *self.id.borrow_mut() = id;
    }
}
