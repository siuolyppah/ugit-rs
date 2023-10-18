use crate::objects::blob::BlobObject;
use crate::objects::manage::{ignored, tracked};
use crate::objects::tree_entry::TreeEntry;
use crate::objects::{sha1_to_string, Object, Sha1Hash, OID};
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
        let tree_obj_content = String::from_utf8(tree_obj_content).unwrap();

        let children = tree_obj_content
            .split(TREE_ENTRY_SEPARATE_STRING)
            .map(TreeEntry::restore_from_str)
            .map(|entry| (entry.origin_file_name(), entry.corresponding_object()))
            .collect();

        Self { children }
    }

    pub fn from_tree_obj_oid(tree_oid: OID) -> Self {
        let (_, obj_content_after_type) = tracked::read_obj_content(tree_oid);



        Self::from_obj_content(obj_content_after_type)
    }

    pub fn push_obj(&mut self, obj: Object, origin_file_name: String) {
        self.children.push((origin_file_name, obj))
    }

    pub fn oid(&self) -> OID {
        sha1_to_string(&self.computed_obj_file_content())
    }

    /// this tree object could be regarded as a virtual file.
    /// the file content is collected from its children.
    ///
    /// # Example
    ///
    /// ```plaintext
    /// blob 91a7b14a584645c7b995100223e65f8a5a33b707 cats.txt
    /// tree 53891a3c27b17e0f8fd96c058f968d19e340428d other
    /// blob fa958e0dd2203e9ad56853a3f51e5945dad317a4 other/dogs.txt
    /// ```
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

    pub fn set_tracked(&self) {
        tracked::track_object(
            &Object::TreeObject(self.clone()).concatenate_flag_and_bytes(),
            self.oid(),
        );

        self.children.iter().for_each(|(_, child)| {
            child.set_tracked();
        })
    }
}

impl Sha1Hash for TreeObject {
    fn sha1(&self) -> OID {
        self.oid()
    }
}
