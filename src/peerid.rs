use multihash::MultiHash;

use key::RSAPubKey;

#[derive(Debug)]
pub enum PeerId {
    Unknown,
    Candidate {
        hash: MultiHash,
    },
    Proven {
        hash: MultiHash,
        key: RSAPubKey,
    }
}

impl PeerId {
    pub fn new(hash: MultiHash, key: RSAPubKey) -> Result<PeerId, ()> {
        if Some(Ok(true)) != hash.validate(key.to_bytes()) {
            return Err(());
        }

        Ok(PeerId::Proven {
            hash: hash,
            key: key,
        })
    }

    pub fn from_key(key: RSAPubKey) -> PeerId {
        PeerId::Proven {
            hash: MultiHash::generate(key.to_bytes()),
            key: key,
        }
    }

    pub fn from_hash(hash: MultiHash) -> PeerId {
        PeerId::Candidate {
            hash: hash,
        }
    }
}
