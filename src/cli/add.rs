use std::path::PathBuf;

/// cmd of `init [PATHSPEC]...`
pub fn cmd_add(pathspec: Vec<PathBuf>) {
    for path in pathspec {
        println!("{:?}", path)
    }
}
