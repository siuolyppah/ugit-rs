//! In Git's lingo, this feature is called "the object database".
//! It allows us to store and retrieve arbitrary blobs, which are called "objects".
//! As far as the Object Database is concerned, the content of the object doesn't have
//! any meaning (just like a filesystem doesn't care about the internal structure of a file).

use std::fmt::Debug;

use crate::objects::tree::TreeObject;
use crate::objects::type_literal::ObjectTypeLiteral;
use crate::{crypto::Sha1HashAble, objects::blob::BlobObject};

use self::db::{insert::ObjectInsert, query, OID};

pub mod blob;
pub mod db;
pub mod manage;
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

impl Sha1HashAble for Object {
    fn sha1(&self) -> OID {
        match self {
            Object::BlobObject(blob) => blob.sha1(),
            Object::TreeObject(tree) => tree.sha1(),
        }
    }
}
