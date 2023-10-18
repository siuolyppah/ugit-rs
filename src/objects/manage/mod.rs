use crate::fs_tools::dirs::check_init;
use crate::fs_tools::files;
use crate::objects::blob::BlobObject;
use crate::objects::tree::TreeObject;
use crate::objects::type_literal::ObjectTypeLiteral;
use crate::objects::{Object, Sha1Hash};
use std::path::{Path, PathBuf};

pub mod ignored;
pub mod tracked;

/// 1. Get the path of the file to store.
/// 2. Read the file.
/// 2. Hash the content of the file using SHA-1.
/// 4. Store the file under ".ugit/objects/{the SHA-1 hash}".
pub fn cmd_hash_object(file: PathBuf, obj_type: ObjectTypeLiteral) {
    check_init();

    // obj_type should be `blob`.
    match obj_type {
        ObjectTypeLiteral::Blob => {
            let contents = files::read_content_to_end(file);
            let blob = BlobObject::new(contents);
            let sha1 = blob.sha1();

            blob.set_tracked();

            println!("{}", sha1)
        }
        _ => unimplemented!(),
    }
}

/// This command is the "opposite" of hash-object: it can print an object by its `oid`.
/// Its implementation just reads the file at `.ugit/objects/{oid}`.
pub fn cmd_cat_file(oid: String, expected_type: ObjectTypeLiteral) {
    check_init();

    let obj = Object::restore_from_file_with_oid(oid);

    match obj {
        Object::BlobObject(blob) => {
            if expected_type != ObjectTypeLiteral::Blob {
                // current type is `Blob`
                panic!(
                    "Expected object type {}, got {}",
                    expected_type,
                    ObjectTypeLiteral::Blob
                );
            } else {
                print!("{}", blob.origin_content());
            }
        }
        Object::TreeObject(tree) => {
            if expected_type != ObjectTypeLiteral::Tree {
                // current type is `Tree`
                panic!(
                    "Expected object type {}, got {}",
                    expected_type,
                    ObjectTypeLiteral::Tree
                );
            } else {
                print!(
                    "{}",
                    String::from_utf8(tree.computed_obj_file_content()).unwrap()
                );
            }
        }
    }
}

/// This command will take the current working directory and store it to the object database.
/// If hash-object was for storing an individual file, then write-tree is for storing a whole directory.
pub fn cmd_write_tree<P: AsRef<Path>>(dir: P) {
    check_init();

    let tree = TreeObject::from_origin_path(dir);
    println!("{}", tree.oid());
}
