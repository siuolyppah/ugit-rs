use crate::fs_tools::dirs::UGIT_REPOSITORY_NAME;
use std::path::Path;

pub fn is_ignored<P: AsRef<Path>>(origin_path: P) -> bool {
    let excludes = vec![UGIT_REPOSITORY_NAME, "target", ".git"];

    let origin_path = origin_path.as_ref().to_str().unwrap();
    excludes.iter().any(|exclude| origin_path.contains(exclude))

    // origin_path
    //     .as_ref()
    //     .to_str()
    //     .unwrap()
    //     .contains(UGIT_REPOSITORY_NAME)
}
