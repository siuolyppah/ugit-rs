//! In Git's lingo, this feature is called "the object database".
//! It allows us to store and retrieve arbitrary blobs, which are called "objects".
//! As far as the Object Database is concerned, the content of the object doesn't have
//! any meaning (just like a filesystem doesn't care about the internal structure of a file).
//!
//! # Object Virtual Content
//!
//! - blob obj:
//! ```plaintext
//! this is content of origin file
//! ```
//!
//! - tree obj:
//! ```plaintext
//! tree a41342933d6cc0b10a0f2c3375eb085e459cf70c f1
//! blob a041bf83d4689c688a8d40ea8e90a6b4753c1af7 root.txt
//! ```
//!
//! # Object File Content
//!
//! - blob obj:
//! ```plaintext
//! blob this is content of origin file
//! ```
//!
//! - tree obj:
//! ```plaintext
//! tree tree a41342933d6cc0b10a0f2c3375eb085e459cf70c f1
//! blob a041bf83d4689c688a8d40ea8e90a6b4753c1af7 root.txt
//! ```
//!
//! # OID
//!
//! calculated by the SHA1 algorithm for *Object File Content*

use std::fmt::Debug;

use crate::objects::tree::TreeObject;
use crate::objects::type_literal::ObjectTypeLiteral;
use crate::{crypto::sha1_to_string, objects::blob::BlobObject};

use self::db::{insert::ObjectInsert, query, OID};
pub use cmd::*;

pub mod blob;
pub mod cmd;
pub mod db;
pub mod ignored;
pub mod tree;
pub mod tree_entry;
pub mod type_literal;

#[derive(Clone, Debug)]
pub enum Object {
    BlobObject(BlobObject),
    TreeObject(TreeObject),
}

pub const TYPE_CONTENT_SEPARATOR: u8 = 0x00;

impl Object {
    /// restore (tree or blob) object with `oid` based on dir `db_path`
    pub fn restore_from_file_with_oid(oid: OID) -> Self {
        let (type_literal, obj_content_after_type) = query::read_object_file(&oid);

        match type_literal {
            ObjectTypeLiteral::Blob => Self::BlobObject(BlobObject::new(obj_content_after_type)),
            ObjectTypeLiteral::Tree => {
                Self::TreeObject(TreeObject::from_obj_content(obj_content_after_type))
            }
        }
    }
}

impl ObjectInsert for Object {
    fn insert_into_db(&self) {
        match self {
            Object::BlobObject(blob) => blob.insert_into_db(),
            Object::TreeObject(tree) => tree.insert_into_db(),
        }
    }
}

pub trait ObjectVirtualContent {
    /// the virtual file contents of obj
    fn obj_virtual_content(&self) -> Vec<u8>;
}

pub trait ObjectFileContent {
    fn obj_file_content(&self) -> Vec<u8>;
}

impl<T> ObjectFileContent for T
where
    T: ObjectVirtualContent,
    ObjectTypeLiteral: for<'a> From<&'a T>,
{
    fn obj_file_content(&self) -> Vec<u8> {
        let mut result = vec![];

        result.extend(ObjectTypeLiteral::from(self).to_string().as_bytes());
        result.push(TYPE_CONTENT_SEPARATOR);

        result.extend(&self.obj_virtual_content());

        result
    }
}

pub trait OidComputable {
    fn oid(&self) -> OID;
}

impl<T> OidComputable for T
where
    T: ObjectFileContent,
{
    fn oid(&self) -> OID {
        sha1_to_string(&self.obj_file_content())
    }
}
