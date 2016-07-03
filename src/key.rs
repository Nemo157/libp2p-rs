use std::fmt;
use openssl::crypto::pkey::PKey;

pub struct RSAPubKey {
    key: PKey,
}

pub struct RSAPrivKey {
    key: PKey,
    pub_key: RSAPubKey,
}

impl RSAPubKey {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.key.save_pub()
    }
}

impl RSAPrivKey {
    pub fn generate() -> RSAPrivKey {
        let mut key = PKey::new();
        key.gen(256);
        RSAPrivKey::from_key(key)
    }

    // TODO: Validate the key has a private key
    fn from_key(priv_key: PKey) -> RSAPrivKey {
        let mut pub_key = PKey::new();
        pub_key.load_pub(&priv_key.save_pub());
        RSAPrivKey { key: priv_key, pub_key: RSAPubKey { key: pub_key } }
    }

    pub fn pub_key(&self) -> &RSAPubKey {
        &self.pub_key
    }
}

impl fmt::Debug for RSAPubKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RSAPubKey")
            .field("key", &self.key.get_rsa())
            .finish()
    }
}

impl fmt::Debug for RSAPrivKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RSAPrivKey")
            .field("key", &self.key.get_rsa())
            .finish()
    }
}
