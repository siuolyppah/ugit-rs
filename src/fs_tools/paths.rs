use std::path::{Path, PathBuf};

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
