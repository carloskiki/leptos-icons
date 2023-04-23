use std::path::{Path, PathBuf};

pub(crate) fn build_crate<P: AsRef<Path>>(relative_path: P) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path)
}

pub(crate) fn library_crate<P1: AsRef<Path>, P2: AsRef<Path>>(
    name: P1,
    relative_path: P2,
) -> PathBuf {
    let mut current = build_crate("");
    current.pop();
    current.join(name).join(relative_path)
}
