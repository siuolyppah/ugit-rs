use crate::fs_tools::files;
use crate::objects::ObjectVirtualContent;
use std::fmt::Debug;
use std::path::Path;

/// origin dir could be restored by `TreeObject`(stored in `TreeVitrualFileEntry`).
/// so, `BlobObject` does not need to hold `ObjectDBPath`.
#[derive(Clone, Debug)]
pub struct BlobObject {
    origin_content: Vec<u8>,
}

impl BlobObject {
    /// origin file content -> Self
    pub fn new(origin_content: Vec<u8>) -> Self {
        Self { origin_content }
    }

    pub fn origin_content(&self) -> &Vec<u8> {
        &self.origin_content
    }

    /// origin file path -> Self
    pub fn from_origin_path<P: AsRef<Path>>(origin_path: P) -> Self {
        Self::new(files::read_content_to_end(origin_path.as_ref()))
    }
}

impl ObjectVirtualContent for BlobObject {
    fn obj_virtual_content(&self) -> Vec<u8> {
        self.origin_content.clone()
    }
}
