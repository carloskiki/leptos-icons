use std::path::{Path, PathBuf};

pub(crate) fn src_path<P: AsRef<Path>>(relative_path: P) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/")
        .join(relative_path)
}

pub(crate) fn crate_path<P: AsRef<Path>>(relative_path: P) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(relative_path)
}
