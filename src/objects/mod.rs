//! In Git's lingo, this feature is called "the object database".
//! It allows us to store and retrieve arbitrary blobs, which are called "objects".
//! As far as the Object Database is concerned, the content of the object doesn't have
//! any meaning (just like a filesystem doesn't care about the internal structure of a file).

use crate::crypto;
use crate::objects::blob::BlobObject;
use crate::objects::manage::tracked;
use crate::objects::tree::TreeObject;
use crate::objects::type_literal::ObjectTypeLiteral;
use std::fmt::{Display, Formatter};

use self::manage::tracked::TrackedInObjectDB;

pub mod blob;
pub mod manage;
pub mod tree;
pub mod tree_entry;
pub mod type_literal;

#[derive(Debug, Clone)]
pub enum Object {
    BlobObject(BlobObject),
    TreeObject(TreeObject),
}

pub const TYPE_CONTENT_SEPARATOR: u8 = 0x00;

impl Object {
    /// restore (tree or blob) object with `oid` based on dir `db_path`
    pub fn restore_from_file_with_oid(oid: OID) -> Self {
        let (type_literal, obj_content_after_type) = tracked::read_obj_content(&oid);

        match type_literal {
            ObjectTypeLiteral::Blob => Self::BlobObject(BlobObject::new(obj_content_after_type)),
            ObjectTypeLiteral::Tree => {
                Self::TreeObject(TreeObject::from_obj_content(obj_content_after_type))
            }
        }
    }

    /// get content that will be stored in object file, the content depands on the obj kind.
    ///
    /// for example:
    /// - blob: content = format!("{type}\x00{origin file content}")
    /// - tree:
    /// ```rs
    /// let entries: Vec<TreeVitrualFileEntry> = todo!();
    /// let virt_file_content = entries.iter()
    ///     .map(|e| e.display())
    ///     .collect<Vec<_>>()
    ///     .join("\n");
    ///
    /// let content = format!("{type}\x00{}", virt_file_content);
    /// ```
    ///
    pub fn bytes_for_sha1(&self) -> Vec<u8> {
        let mut result = vec![];

        result.extend(ObjectTypeLiteral::from(self.clone()).to_string().as_bytes());
        result.push(TYPE_CONTENT_SEPARATOR);

        match self {
            Object::BlobObject(blob) => {
                result.extend(blob.origin_content().bytes());
            }
            Object::TreeObject(tree) => {
                result.extend(&tree.computed_obj_file_content());
            }
        }

        result
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::BlobObject(blob) => write!(f, "{}", blob.origin_content()),
            Object::TreeObject(tree) => write!(
                f,
                "{}",
                String::from_utf8(tree.computed_obj_file_content()).unwrap()
            ),
        }
    }
}

impl TrackedInObjectDB for Object {
    fn save_object_file(&self) {
        match self {
            Object::BlobObject(blob) => blob.save_object_file(),
            Object::TreeObject(tree) => tree.save_object_file(),
        }
    }
}

type OID = String;

trait Sha1Hash {
    fn sha1(&self) -> OID;
}

fn sha1_to_string(buf: &Vec<u8>) -> String {
    let hash_val = crypto::sha1(buf);
    hex::encode(&hash_val)
}
