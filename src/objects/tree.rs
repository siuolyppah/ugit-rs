use crate::fs_tools::{files, paths};
use crate::objects::blob::BlobObject;
use crate::objects::tree_entry::TreeVitrualFileEntry;
use crate::objects::type_literal::ObjectTypeLiteral;
use crate::objects::{Object, OID};
use std::fmt::{Debug, Formatter};
use std::fs;
use std::path::Path;

use super::db::insert::{self, ObjectInsert};
use super::db::query;
use super::db::restore::ObjectRestore;
use super::{ignored, ObjectFileContent, ObjectVirtualContent, OidComputable};

pub const TREE_ENTRY_SEPARATE_STRING: &str = "\n";

/// a `TreeObject` corresponding a list of `TreeVitrualFileEntry`.
///
/// # how to restore file origin path, for given `oid`:
///
/// 1. read content of treeobj located in `{UGIT_REPOSITORY_NAME}/oid`;
/// 2. restore `Vec<TreeVitrualFileEntry>`;
/// 3.
#[derive(Default, Clone)]
pub struct TreeObject {
    /// `(origin_file_name, Object)`
    children: Vec<(String, Object)>,
}

impl Debug for TreeObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        #[derive(Debug)]
        #[allow(dead_code)]
        enum HumanReadable<'a> {
            File {
                origin_relative_path: &'a String,
                origin_content: String,
            },
            Dir {
                origin_relative_path: &'a String,
                its_oid: String,
            },
        }

        let human_readable_children = self
            .children
            .iter()
            .map(|(origin_relative_path, obj)| match obj {
                Object::BlobObject(blob) => HumanReadable::File {
                    origin_relative_path,
                    origin_content: String::from_utf8(blob.origin_content().to_owned()).unwrap(),
                },
                Object::TreeObject(tree) => HumanReadable::Dir {
                    origin_relative_path,
                    its_oid: tree.oid(),
                },
            })
            .collect::<Vec<_>>();

        f.debug_struct("TreeObject")
            .field("children", &human_readable_children)
            .finish()
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
    /// # PERF: expensive, this fn was impled by walkthrough the `origin_root_dir`
    ///
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
        let (obj_literal, obj_content_after_type) = query::read_object_file(&tree_oid);

        if obj_literal != ObjectTypeLiteral::Tree {
            panic!("the object of given oid {} is not a tree-object", tree_oid);
        }

        Self::from_obj_content(obj_content_after_type)
    }
}

impl ObjectVirtualContent for TreeObject {
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
    fn obj_virtual_content(&self) -> Vec<u8> {
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

impl ObjectInsert for TreeObject {
    fn insert_into_db(&self) {
        insert::save_into_object_file(&self.obj_file_content(), &self.oid());

        self.children.iter().for_each(|(_, child)| {
            child.insert_into_db();
        })
    }
}

impl TreeObject {
    fn _restore(&self) {
        for (origin_file_name, obj) in &self.children {
            match obj {
                Object::BlobObject(blob) => {
                    files::store_file(origin_file_name, &blob.origin_content())
                }
                Object::TreeObject(tree) => {
                    tree._restore();
                }
            }
        }
    }
}

impl ObjectRestore for TreeObject {
    fn restore(&self) {
        paths::empty_cwd();
        self._restore();
    }
}
