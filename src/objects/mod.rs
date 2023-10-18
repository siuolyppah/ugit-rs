//! In Git's lingo, this feature is called "the object database".
//! It allows us to store and retrieve arbitrary blobs, which are called "objects".
//! As far as the Object Database is concerned, the content of the object doesn't have
//! any meaning (just like a filesystem doesn't care about the internal structure of a file).

use std::{ops::Deref, path::PathBuf};

use crate::fs_tools::{dirs, files};

use self::git_obj::{GitObject, GitObjectType};

pub mod git_obj;

/// 1. Get the path of the file to store.
/// 2. Read the file.
/// 2. Hash the content of the file using SHA-1.
/// 4. Store the file under ".ugit/objects/{the SHA-1 hash}".
pub fn cmd_hash_object(file: PathBuf, obj_type: GitObjectType) {
    if !dirs::is_objects_dir_exist() {
        eprintln!(
            "fatal: not a ugit repository (or any of the parent directories): {}",
            dirs::UGIT_REPOSITORY_NAME
        );

        return;
    }

    let contents = files::read_content_to_end(file);
    let git_obj = GitObject::new(&contents, obj_type);

    let oid = git_obj.get_oid_by_sha1();

    store_object(&oid, &git_obj.collect_bytes());

    println!("{}", oid);
}

/// This command is the "opposite" of hash-object: it can print an object by its `oid`.
/// Its implementation just reads the file at `.ugit/objects/{oid}`.
pub fn cmd_cat_file(oid: String, expected_type: GitObjectType) {
    let obj_path = format!("{}/{}", &dirs::OBJECTS_DIR_PATH.deref(), oid);
    let file_contents = files::read_content_to_end(obj_path);

    match GitObject::try_from(&file_contents[..]) {
        Ok(git_obj) => {
            if git_obj.obj_type() != expected_type {
                eprintln!(
                    "expected obj type {:?}, got {:?}",
                    expected_type,
                    git_obj.obj_type()
                );
            } else {
                print!("{}", git_obj.string_of_content());
            }
        }
        Err(e) => {
            eprintln!("{:?}", e)
        }
    }
}

pub fn store_object(oid: &str, u8s: &[u8]) {
    let obj_path = format!("{}/{}", &dirs::OBJECTS_DIR_PATH.deref(), oid);
    files::store_file(obj_path, u8s)
}
