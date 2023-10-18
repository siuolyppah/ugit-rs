use std::{ops::Deref, path::PathBuf};

use crate::fs_tools::dirs;

pub mod insert;
pub mod query;

pub type OID = String;

#[inline]
pub fn object_save_path_for_oid(oid: &OID) -> PathBuf {
    PathBuf::from(format!("{}/{}", &dirs::OBJECTS_DIR_PATH.deref(), oid))
}
