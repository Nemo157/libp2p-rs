use multihash::MultiHash;

use key::RSAPrivKey;

#[derive(Debug)]
pub struct HostId {
    hash: MultiHash,
    key: RSAPrivKey,
}

impl HostId {
    pub fn new(hash: MultiHash, key: RSAPrivKey) -> Result<HostId, ()> {
        if Some(Ok(true)) != hash.validate(key.pub_key().to_bytes()) {
            return Err(());
        }

        Ok(HostId {
            hash: hash,
            key: key,
        })
    }

    pub fn generate() -> HostId {
        HostId::from_key(RSAPrivKey::generate())
    }

    pub fn from_key(key: RSAPrivKey) -> HostId {
        HostId {
            hash: MultiHash::generate(key.pub_key().to_bytes()),
            key: key,
        }
    }
}
