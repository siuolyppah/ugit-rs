use crate::{
    fs_tools::files,
    objects::{blob::BlobObject, ObjectFileContent, OidComputable},
};

use super::{object_save_path_for_oid, OID};

pub trait ObjectInsert {
    fn insert_into_db(&self);
}

pub fn save_into_object_file(content: &[u8], oid: &OID) {
    let obj_path = object_save_path_for_oid(oid);

    files::store_file(obj_path, &content)
}

macro_rules! impl_object_insert {
    ($object_type: ty) => {
        impl ObjectInsert for $object_type {
            fn insert_into_db(&self) {
                save_into_object_file(&self.obj_file_content(), &self.oid())
            }
        }
    };
}

impl_object_insert!(BlobObject);
