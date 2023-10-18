use crate::fs_tools::files;
use crate::objects::manage::tracked;
use crate::objects::{sha1_to_string, Object, Sha1Hash, OID};
use std::path::Path;

use super::manage::tracked::TrackedInObjectDB;

/// origin dir could be restored by `TreeObject`(stored in `TreeVitrualFileEntry`).
/// so, `BlobObject` does not need to hold `ObjectDBPath`.
#[derive(Debug, Clone)]
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
        obj.oid = Some(sha1_to_string(
            &Object::BlobObject(obj.clone()).bytes_for_sha1(),
        ));

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
}

impl Sha1Hash for BlobObject {
    fn sha1(&self) -> OID {
        self.oid()
    }
}

impl TrackedInObjectDB for BlobObject {
    fn save_object_file(&self) {
        tracked::track_object(
            &self.origin_content,
            &self.oid(),
        )
    }
}
