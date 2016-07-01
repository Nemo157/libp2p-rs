use multihash::MultiHash;

pub type RSAPubKey = (); // TODO: Sort out a real key type
pub type RSAPrivKey = (); // TODO: Sort out a real key type

pub struct HostId {
    hash: MultiHash,
    pub_key: RSAPubKey,
    priv_key: RSAPrivKey,
}

impl HostId {
    pub fn new(hash: MultiHash, pub_key: RSAPubKey, priv_key: RSAPrivKey) -> Result<HostId, ()> {
        if Some(Ok(true)) != hash.validate(&[] /* pub_key.as_bytes() */) {
            return Err(());
        }

        Ok(HostId {
            hash: hash,
            pub_key: pub_key,
            priv_key: priv_key,
        })
    }

    pub fn from_keys(pub_key: RSAPubKey, priv_key: RSAPrivKey) -> HostId {
        HostId {
            hash: MultiHash::generate(&[] /* pub_key.as_bytes() */),
            pub_key: pub_key,
            priv_key: priv_key,
        }
    }
}
