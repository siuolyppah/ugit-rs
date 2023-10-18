use sha1::{Digest, Sha1};

pub fn sha1(buf: &Vec<u8>) -> [u8; 20] {
    // TODO: use global hasher
    let mut hasher = Sha1::new();

    hasher.update(buf);
    hasher.finalize().into()
}
