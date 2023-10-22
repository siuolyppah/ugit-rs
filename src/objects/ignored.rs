use crate::fs_tools::dirs::UGIT_REPOSITORY_NAME;
use std::path::Path;

pub fn is_ignored<P: AsRef<Path>>(origin_path: P) -> bool {
    origin_path
        .as_ref()
        .to_str()
        .unwrap()
        .contains(UGIT_REPOSITORY_NAME)
}
