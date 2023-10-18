use sha1::{Digest, Sha1};

use crate::objects::db::OID;

pub fn sha1_to_array(buf: &Vec<u8>) -> [u8; 20] {
    // TODO: use global hasher
    let mut hasher = Sha1::new();

    hasher.update(buf);
    hasher.finalize().into()
}

pub fn sha1_to_string(buf: &Vec<u8>) -> String {
    let hash_val = sha1_to_array(buf);
    hex::encode(&hash_val)
}

pub trait Sha1HashAble {
    fn sha1(&self) -> OID;
}
