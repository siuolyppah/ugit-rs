use std::{
    env, fs,
    path::{Path, PathBuf},
};

use super::dirs::UGIT_REPOSITORY_NAME;

/// return the suffix of `to_check`, based on `base`.
///
/// `base` and `to_check` could be relative path.
/// if so, the absolute path of both are compared.
///
/// Returns [`None`] if the `base` is not the parent of `to_check`.
pub fn suffix_of<P1: AsRef<Path>, P2: AsRef<Path>>(base: P1, to_check: P2) -> Option<PathBuf> {
    // TODO: error handle
    let absolute_base = base.as_ref().canonicalize().unwrap();
    let absolute_to_check = to_check.as_ref().canonicalize().unwrap();

    match absolute_to_check.strip_prefix(absolute_base) {
        Ok(suffix) => Some(suffix.into()),
        Err(_) => None,
    }
}

pub fn is_parent_or_same_directory(parent: &Path, child: &Path) -> bool {
    let parent_components: Vec<_> = parent.components().collect();
    let child_components: Vec<_> = child.components().collect();

    for (a_comp, b_comp) in parent_components.iter().zip(child_components.iter()) {
        if a_comp != b_comp {
            return false;
        }
    }

    child_components.len() >= parent_components.len()
}

/// all files and directories in the current work directory will
/// be deleted except the folder `UGIT_REPOSITORY_NAME`
pub fn empty_cwd() {
    let cwd = env::current_dir().unwrap();

    if cfg!(debug_assertions) {
        // preventing the deletion of the source code directory
        let project_root = env!("CARGO_MANIFEST_DIR");
        let project_root = PathBuf::from(project_root);

        if is_parent_or_same_directory(&project_root, &cwd) {
            // your cwd is under project root. careful.

            println!("your current working directory is under project root. nothing was deleted. Be Careful.")
        }
    } else {
        delete_all_under(
            &cwd,
            // TODO
            vec![UGIT_REPOSITORY_NAME, ".git", ".gitignore"],
        );
    }
}

fn delete_all_under(root: &Path, excludes: Vec<&str>) {
    fs::read_dir(root)
        .unwrap()
        .map(|entry| entry.unwrap())
        .filter(|entry| {
            let binding = entry.path();
            let entry_str = binding.as_path().as_os_str().to_str().unwrap();

            excludes.iter().all(|exclude| !entry_str.contains(exclude))

            // !excludes.contains(&e.path().as_path())
        })
        .for_each(|entry| {
            let path = entry.path();
            if entry.path().is_dir() {
                fs::remove_dir_all(path).unwrap();
            } else {
                fs::remove_file(path).unwrap();
            }
        })
}
