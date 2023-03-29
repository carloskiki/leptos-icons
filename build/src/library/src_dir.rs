use std::path::PathBuf;

use anyhow::Result;
use tracing::info;

use super::{icon_module::IconModule, lib_rs::LibRs};

#[derive(Debug)]
pub(crate) struct SrcDir {
    pub path: PathBuf,
    pub lib_rs: LibRs,
    pub icon_modules: Vec<IconModule>,
}

impl SrcDir {
    /// Removes everything inside and creates a fresh lib.rs file.
    pub async fn reset(&mut self) -> Result<()> {
        info!(path = ?self.path, "Removing existing src directory");
        tokio::fs::remove_dir_all(&self.path).await?;

        info!(path = ?self.path, "Creating new src directory");
        tokio::fs::create_dir(&self.path).await?;

        self.lib_rs.init().await?;
        Ok(())
    }

    pub fn lib_rs(&self) -> &LibRs {
        &self.lib_rs
    }

    pub fn add_module(&mut self, module: IconModule) -> &IconModule {
        self.icon_modules.push(module);
        self.icon_modules.last().expect("must exist")
    }
}
