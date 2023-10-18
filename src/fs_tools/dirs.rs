use once_cell::sync::Lazy;
use std::{fs, io, ops::Deref, path::Path};

pub const UGIT_REPOSITORY_NAME: &str = ".ugit";
pub const OBJECTS_DIR_NAME: &str = "objects";
pub static OBJECTS_DIR_PATH: Lazy<String> =
    Lazy::new(|| format!("{}/{}", UGIT_REPOSITORY_NAME, OBJECTS_DIR_NAME));

/// create dir `{cwd}/{UGIT_REPOSITORY_NAME}`
pub fn create_repo_dir() -> io::Result<()> {
    fs::create_dir(UGIT_REPOSITORY_NAME)
}

/// create dir `{cwd}/{UGIT_REPOSITORY_NAME}/{OBJECTS_DIR_NAME}`
pub fn create_objects_dir() -> io::Result<()> {
    fs::create_dir(&OBJECTS_DIR_PATH.deref())
}

pub fn is_repo_dir_exist() -> bool {
    Path::new(&UGIT_REPOSITORY_NAME).is_dir()
}

pub fn is_objects_dir_exist() -> bool {
    Path::new(&UGIT_REPOSITORY_NAME).is_dir()
}
