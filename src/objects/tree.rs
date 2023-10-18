use crate::fs_tools::paths;
use crate::objects::blob::BlobObject;
use crate::objects::manage::{ignored, tracked};
use crate::objects::tree_entry::TreeVitrualFileEntry;
use crate::objects::type_literal::ObjectTypeLiteral;
use crate::objects::{sha1_to_string, Object, Sha1Hash, OID};
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::Path;

use super::manage::tracked::TrackedInObjectDB;

pub const TREE_ENTRY_SEPARATE_STRING: &str = "\n";

/// a `TreeObject` corresponding a list of `TreeVitrualFileEntry`.
///
/// # how to restore file origin path, for given `oid`:
///
/// 1. read content of treeobj located in `{UGIT_REPOSITORY_NAME}/oid`;
/// 2. restore `Vec<TreeVitrualFileEntry>`;
/// 3.
#[derive(Debug, Default, Clone)]
pub struct TreeObject {
    /// `(origin_file_name, Object)`
    children: Vec<(String, Object)>,
}

impl Display for TreeObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (origin_file_name, obj) in &self.children {
            writeln!(f, "{}:", origin_file_name)?;
            writeln!(f, "---obj begin")?;
            write!(f, "{}", obj)?;
            writeln!(f, "---obj end")?;
        }

        Ok(())
    }
}

impl TreeObject {
    fn _from_origin_dir<P: AsRef<Path>>(visiting: P, root: &Path) -> Self {
        let children = fs::read_dir(visiting.as_ref())
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| !ignored::is_ignored(path))
            .map(|path_under_visiting| {
                let suffix = match paths::suffix_of(root, path_under_visiting.clone()) {
                    Some(suffix) => suffix,
                    None => unreachable!(),
                };

                let origin_file_name_of_suffix = suffix.as_path().to_str().unwrap().to_owned();

                if path_under_visiting.is_dir() {
                    let tree_obj = Self::_from_origin_dir(path_under_visiting.clone(), root);
                    (origin_file_name_of_suffix, Object::TreeObject(tree_obj))
                } else {
                    // should be file
                    let blob_obj = BlobObject::from_origin_path(path_under_visiting.clone());
                    (origin_file_name_of_suffix, Object::BlobObject(blob_obj))
                }
            })
            .collect();

        Self { children }
    }

    /// dir(and files in it) with given path -> Self
    ///
    /// # PERF: expensive, this fn was impled by walkthrough the `origin_root_dir`.
    pub fn from_origin_dir<P: AsRef<Path>>(origin_root_dir: P) -> Self {
        Self::_from_origin_dir(origin_root_dir.as_ref(), origin_root_dir.as_ref())
    }

    pub fn from_obj_content(tree_obj_content: Vec<u8>) -> Self {
        // tree obj content must be UTF-8 chars.
        let tree_obj_content = String::from_utf8(tree_obj_content).unwrap();

        let children = tree_obj_content
            .split(TREE_ENTRY_SEPARATE_STRING)
            .map(TreeVitrualFileEntry::restore_from_str)
            .map(|entry| (entry.origin_file_name(), entry.corresponding_object()))
            .collect();

        Self { children }
    }

    pub fn from_tree_obj_oid(tree_oid: OID) -> Self {
        let (obj_literal, obj_content_after_type) = tracked::read_obj_content(&tree_oid);

        if obj_literal != ObjectTypeLiteral::Tree {
            panic!("the object of given oid {} is not a tree-object", tree_oid);
        }

        Self::from_obj_content(obj_content_after_type)
    }

    pub fn oid(&self) -> OID {
        sha1_to_string(&self.computed_obj_file_content())
    }

    /// tree object could be regarded as a virtual file.
    /// the file content is collected from its children objects.
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
                TreeVitrualFileEntry::from_obj(obj.clone(), origin_file_name.clone()).to_string()
            })
            .collect::<Vec<_>>()
            .join(TREE_ENTRY_SEPARATE_STRING)
            .as_bytes()
            .to_vec()
    }
}

impl Sha1Hash for TreeObject {
    fn sha1(&self) -> OID {
        self.oid()
    }
}

impl TrackedInObjectDB for TreeObject {
    /// when tree object was saved in file, the structure was **flatten**.
    fn save_object_file(&self) {
        tracked::track_object(
            &self.computed_obj_file_content(),
            &self.oid(),
        );

        self.children.iter().for_each(|(_, child)| {
            child.save_object_file();
        })
    }
}
