use crate::fs_tools::files;
use crate::objects::manage::tracked;
use crate::objects::{sha1_to_string, ObjectTypeLiteral, Sha1Hash, OID};
use std::path::Path;

pub const TYPE_CONTENT_SEPARATOR: u8 = 0x00;

#[derive(Debug, Clone)]
pub struct BlobObject {
    origin_content: Vec<u8>,
    oid: Option<OID>,
}

impl BlobObject {
    pub fn new(origin_content: Vec<u8>) -> Self {
        let mut obj = Self {
            origin_content,
            oid: None,
        };
        obj.oid = Some(sha1_to_string(&obj.concatenate_flag_and_bytes()));
        obj
    }

    pub fn origin_content(&self) -> String {
        String::from_utf8(self.origin_content.clone()).unwrap()
    }

    pub fn from_origin_path<P: AsRef<Path>>(origin_path: P) -> Self {
        Self::new(files::read_content_to_end(origin_path))
    }

    pub fn concatenate_flag_and_bytes(&self) -> Vec<u8> {
        let mut result = vec![];

        result.extend(ObjectTypeLiteral::from(self).to_string().as_bytes());
        result.push(TYPE_CONTENT_SEPARATOR);
        result.extend(&self.origin_content);

        result
    }

    pub fn oid(&self) -> OID {
        match &self.oid {
            None => {
                unreachable!()
            }
            Some(oid) => oid.clone(),
        }
    }

    pub fn set_tracked(&self) {
        tracked::track_object(&self.concatenate_flag_and_bytes(), self.oid())
    }
}

impl Sha1Hash for BlobObject {
    fn sha1(&self) -> OID {
        self.oid()
    }
}
