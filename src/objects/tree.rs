use crate::objects::blob::BlobObject;
use crate::objects::manage::ignored;
use crate::objects::tree_entry::TreeEntry;
use crate::objects::{sha1_to_string, Object, OID};
use std::fs;
use std::path::Path;

pub const TREE_ENTRY_SEPARATE_STRING: &str = "\n";

#[derive(Debug, Default, Clone)]
pub struct TreeObject {
    /// `(origin_file_name, Object)`
    children: Vec<(String, Object)>,
}

impl TreeObject {
    pub fn from_origin_path<P: AsRef<Path>>(origin_path: P) -> Self {
        let children = fs::read_dir(origin_path)
            .unwrap()
            .into_iter()
            .map(|entry| entry.unwrap().path())
            .filter(|path| !ignored::is_ignored(path))
            .map(|path| {
                if path.is_dir() {
                    let tree_obj = Self::from_origin_path(path.clone());
                    (
                        path.file_name()
                            .unwrap()
                            .to_os_string()
                            .into_string()
                            .unwrap(),
                        Object::TreeObject(tree_obj),
                    )
                } else {
                    // should be file
                    let blob_obj = BlobObject::from_origin_path(path.clone());
                    (
                        path.file_name()
                            .unwrap()
                            .to_os_string()
                            .into_string()
                            .unwrap(),
                        Object::BlobObject(blob_obj),
                    )
                }
            })
            .collect();

        Self { children }
    }

    pub fn from_obj_content(tree_obj_content: Vec<u8>) -> Self {
        unimplemented!()
    }

    pub fn push_obj(&mut self, obj: Object, origin_file_name: String) {
        self.children.push((origin_file_name, obj))
    }

    pub fn oid(&self) -> OID {
        sha1_to_string(&self.computed_obj_file_content())
    }

    pub fn computed_obj_file_content(&self) -> Vec<u8> {
        self.children
            .iter()
            .map(|(origin_file_name, obj)| {
                TreeEntry::from_obj(obj.clone(), origin_file_name.clone()).to_string()
            })
            .collect::<Vec<_>>()
            .join(TREE_ENTRY_SEPARATE_STRING)
            .as_bytes()
            .to_vec()
    }
}
