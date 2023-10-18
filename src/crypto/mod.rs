use sha1::{Digest, Sha1};

pub fn sha1(u8s: &Vec<u8>) -> [u8; 20] {
    let mut hasher = Sha1::new();

    hasher.update(u8s);
    hasher.finalize().into()
}
