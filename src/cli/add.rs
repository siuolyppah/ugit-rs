use std::path::PathBuf;

use crate::fs_tools::dirs;

/// cmd of `init [PATHSPEC]...`
pub fn cmd_add(pathspec: Vec<PathBuf>) {
    dirs::check_init();

    for path in pathspec {
        println!("{:?}", path)
    }
}
