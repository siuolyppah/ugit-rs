use crate::fs_tools::{dirs, files};
use crate::objects::blob::TYPE_CONTENT_SEPARATOR;
use crate::objects::OID;
use std::ops::Deref;
use std::path::PathBuf;

#[inline]
pub fn object_save_path(oid: OID) -> PathBuf {
    PathBuf::from(format!("{}/{}", &dirs::OBJECTS_DIR_PATH.deref(), oid))
}

pub fn track_object(content: &[u8], oid: OID) {
    let obj_path = object_save_path(oid);

    files::store_file(obj_path, &content)
}

/// return `(type literal, obj file content after type literal)`
pub fn read_obj_content(oid: OID) -> (String, Vec<u8>) {
    let blob_content = files::read_content_to_end(object_save_path(oid));

    if let Some(sep_idx) = blob_content
        .iter()
        .position(|&x| x == TYPE_CONTENT_SEPARATOR)
    {
        let (type_literal, origin_contents) = blob_content.split_at(sep_idx);

        (
            String::from_utf8(type_literal.to_vec()).unwrap(),
            origin_contents.to_vec(),
        )
    } else {
        panic!("unrecognized object file format.")
    }
}
