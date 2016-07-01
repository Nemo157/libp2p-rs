use multihash::MultiHash;

pub type RSAPubKey = (); // TODO: Sort out a real key type

pub struct PeerId {
    hash: MultiHash,
    key: RSAPubKey,
}

impl PeerId {
    pub fn new(hash: MultiHash, key: RSAPubKey) -> Result<PeerId, ()> {
        if Some(Ok(true)) != hash.validate(&[] /* key.as_bytes() */) {
            return Err(());
        }

        Ok(PeerId {
            hash: hash,
            key: key,
        })
    }

    pub fn from_key(key: RSAPubKey) -> PeerId {
        PeerId {
            hash: MultiHash::generate(&[] /* key.as_bytes() */),
            key: key,
        }
    }
}
