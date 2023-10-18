use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

pub fn read_content_to_end<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = File::open(&path).expect("could not open file");

    let mut buf = vec![];
    file.read_to_end(&mut buf).expect("could not read to end");

    buf
}

pub fn store_file<P: AsRef<Path>>(path: P, u8s: &[u8]) {
    let mut file = File::create(path).expect("create file fail.");

    file.write_all(u8s).expect("write to file fail.");

    file.flush().expect("flush fail.");
}
