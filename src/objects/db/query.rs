use crate::{
    fs_tools::files,
    objects::{type_literal::ObjectTypeLiteral, TYPE_CONTENT_SEPARATOR},
};

use super::{object_save_path_for_oid, OID};

pub trait ObjectQuery {
    fn query_db_with_oid(oid: OID) -> Self;
}

/// return `(type, obj file content after type literal)`
pub fn read_object_file(oid: &OID) -> (ObjectTypeLiteral, Vec<u8>) {
    let obj_file_content = files::read_content_to_end(object_save_path_for_oid(oid));

    if let Some(sep_idx) = obj_file_content
        .iter()
        .position(|&x| x == TYPE_CONTENT_SEPARATOR)
    {
        let (type_literal, mut origin_contents) = obj_file_content.split_at(sep_idx);

        let type_literal = match ObjectTypeLiteral::try_from(
            String::from_utf8(type_literal.to_vec()).unwrap().as_str(),
        ) {
            Ok(type_literal) => type_literal,
            Err(_) => {
                panic!("unknown obj type literal")
            }
        };

        // skip `0x00`
        origin_contents = &origin_contents[1..];

        (type_literal, origin_contents.to_vec())
    } else {
        panic!("unrecognized object file format.")
    }
}
