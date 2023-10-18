use crate::fs_tools::files;

use super::{object_save_path_for_oid, OID};

pub trait ObjectInsert {
    fn insert_into_db(&self);
}

pub fn save_into_object_file(content: &[u8], oid: &OID) {
    let obj_path = object_save_path_for_oid(oid);

    files::store_file(obj_path, &content)
}
