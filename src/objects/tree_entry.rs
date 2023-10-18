use crate::objects::blob::BlobObject;
use crate::objects::tree::TreeObject;
use crate::objects::type_literal::ObjectTypeLiteral;
use crate::objects::{Object, OID};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct TreeEntry {
    obj_type: ObjectTypeLiteral,
    oid: OID,
    origin_file_name: String,
}

impl Display for TreeEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.obj_type,
            self.oid,
            self.origin_file_name
        )
    }
}

impl TreeEntry {
    pub fn from_blob_obj(blob: BlobObject, origin_file_name: String) -> Self {
        Self {
            obj_type: ObjectTypeLiteral::Blob,
            oid: blob.oid(),
            origin_file_name,
        }
    }

    pub fn from_tree_obj(tree: TreeObject, origin_file_name: String) -> Self {
        Self {
            obj_type: ObjectTypeLiteral::Tree,
            oid: tree.oid(),
            origin_file_name,
        }
    }

    pub fn from_obj(obj: Object, origin_file_name: String) -> Self {
        match obj {
            Object::BlobObject(obj) => Self::from_blob_obj(obj, origin_file_name),
            Object::TreeObject(obj) => Self::from_tree_obj(obj, origin_file_name),
        }
    }

    pub fn restore_from_str(entry_str: &str) -> Self {
        let mut fields = entry_str.split_whitespace();

        let obj_type_str = fields.next().unwrap();
        let oid_str = fields.next().unwrap();
        let origin_file_name = fields.next().unwrap();

        if fields.next().is_some() {
            panic!("unrecognized object tree entry format.");
        }

        if let Ok(type_literal) = ObjectTypeLiteral::try_from(obj_type_str) {
            Self {
                obj_type: type_literal,
                oid: oid_str.to_string(),
                origin_file_name: origin_file_name.to_string(),
            }
        } else {
            panic!("unknown type literal.")
        }
    }

    pub fn origin_file_name(&self) -> String{
        self.origin_file_name.clone()
    }

    pub fn corresponding_object(&self) -> Object{
        Object::restore_from_file_with_oid(self.oid.clone())
    }
}
