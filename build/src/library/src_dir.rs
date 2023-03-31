use std::path::PathBuf;

use anyhow::Result;
use tokio::io::AsyncWriteExt;
use tracing::{error, info};

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

    pub fn add_module(&mut self, module: IconModule) {
        self.icon_modules.push(module);
        // Keeping modules sorted to avoid churn.
        self.icon_modules.sort_by(|a, b| a.name().cmp(b.name()));
    }

    pub fn modules(&self) -> &[IconModule] {
        &self.icon_modules
    }

    pub async fn write_module_declarations(&mut self) -> Result<()> {
        let mut writer = self.lib_rs.append().await?;
        for module in self.modules() {
            writer.write_all("pub mod ".as_bytes()).await?; // TODO: Let this depend on mod visibility.
            writer.write_all(module.name().as_bytes()).await?;
            writer.write_all(";\n".as_bytes()).await?;
        }
        writer.flush().await.map_err(|err| {
            error!(?err, "Could not flush lib.rs file after writing.");
            err
        })?;
        Ok(())
    }
}
