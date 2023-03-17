use std::process::{Command, ExitStatus};
use crate::types::IconPackage;
use anyhow::Result;

pub(crate) fn download_submodule(icon_package: &IconPackage) -> Result<ExitStatus> {
    Command::new("git")
        .args([
            "submodule",
            "add",
            "-f",
            "--depth",
            "1",
            icon_package.url.as_str(),
            &format!("./icons/{}", icon_package.folder_name),
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .map_err(anyhow::Error::from)
}
