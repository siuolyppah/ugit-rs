use crate::crypto::{sha1_to_string, Sha1HashAble};
use crate::fs_tools::files;
use crate::objects::OID;
use std::fmt::Debug;
use std::path::Path;

use super::db::insert::{self, ObjectInsert};
use super::type_literal::ObjectTypeLiteral;
use super::TYPE_CONTENT_SEPARATOR;

/// origin dir could be restored by `TreeObject`(stored in `TreeVitrualFileEntry`).
/// so, `BlobObject` does not need to hold `ObjectDBPath`.
#[derive(Clone, Debug)]
pub struct BlobObject {
    origin_content: Vec<u8>,

    // `oid` was calculated only when creatation.
    // after creatation, oid should always be `Some(oid)`.
    oid: Option<OID>,
}

impl BlobObject {
    /// origin file content -> Self
    pub fn new(origin_content: Vec<u8>) -> Self {
        let mut obj = Self {
            origin_content,
            oid: None,
        };

        // this is tricky, `oid` will be calculated right now,
        // and the hash val will be cached.
        obj.oid = Some(sha1_to_string(&obj.bytes_for_sha1()));

        obj
    }

    pub fn origin_content(&self) -> String {
        String::from_utf8(self.origin_content.clone()).unwrap()
    }

    /// origin file path -> Self
    pub fn from_origin_path<P: AsRef<Path>>(origin_path: P) -> Self {
        Self::new(files::read_content_to_end(origin_path.as_ref()))
    }

    pub fn oid(&self) -> OID {
        match &self.oid {
            None => {
                unreachable!("inited after creatation.")
            }
            Some(oid) => oid.clone(),
        }
    }

    /// get bytes that will be *stored in object file* or *hashed by SHA1*.
    pub fn bytes_for_sha1(&self) -> Vec<u8> {
        let mut result = vec![];

        result.extend(ObjectTypeLiteral::Blob.to_string().as_bytes());
        result.push(TYPE_CONTENT_SEPARATOR);

        result.extend(self.origin_content().bytes());

        result
    }
}

impl ObjectInsert for BlobObject {
    fn insert_into_db(&self) {
        insert::save_into_object_file(&self.bytes_for_sha1(), &self.oid())
    }
}

impl Sha1HashAble for BlobObject {
    fn sha1(&self) -> OID {
        self.oid()
    }
}
