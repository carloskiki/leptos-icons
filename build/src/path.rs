use std::path::{Path, PathBuf};

pub(crate) fn build_crate<P: AsRef<Path>>(relative_path: P) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path)
}

pub(crate) fn download_path<P: AsRef<Path>>(relative_path: P) -> PathBuf {
    build_crate("downloads").join(relative_path)
}

pub(crate) fn leptos_icons_crate<P: AsRef<Path>>(relative_path: P) -> PathBuf {
    let mut current = build_crate("");
    current.pop();
    current.join("leptos-icons").join(relative_path)
}

pub(crate) fn src_path<P: AsRef<Path>>(relative_path: P) -> PathBuf {
    leptos_icons_crate("src").join(relative_path)
}
