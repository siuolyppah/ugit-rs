//! In Git's lingo, this feature is called "the object database".
//! It allows us to store and retrieve arbitrary blobs, which are called "objects".
//! As far as the Object Database is concerned, the content of the object doesn't have
//! any meaning (just like a filesystem doesn't care about the internal structure of a file).

use std::{ops::Deref, path::PathBuf};

use crate::{
    crypto,
    fs_tools::{dirs, files},
};

/// 1. Get the path of the file to store.
/// 2. Read the file.
/// 2. Hash the content of the file using SHA-1.
/// 4. Store the file under ".ugit/objects/{the SHA-1 hash}".
pub fn cmd_hash_object(file: PathBuf) {
    if !dirs::is_objects_dir_exist() {
        eprintln!(
            "fatal: not a ugit repository (or any of the parent directories): {}",
            dirs::UGIT_REPOSITORY_NAME
        );

        return;
    }

    let contents = files::read_content_to_end(file);
    let oid = get_oid(&contents);

    store_object(&oid, &contents);

    println!("{}", oid);
}

/// hexadecimal representation of the result of the SHA-1 hash.
///
/// "OID" - object ID
pub fn get_oid(u8s: &Vec<u8>) -> String {
    let hash_val = crypto::sha1(&u8s);

    hex::encode(&hash_val)
}

pub fn store_object(oid: &str, u8s: &[u8]) {
    let obj_path = format!("{}/{}", &dirs::OBJECTS_DIR_PATH.deref(), oid);
    files::store_file(obj_path, u8s)
}
