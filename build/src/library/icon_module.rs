use std::path::{Path, PathBuf};

use anyhow::Result;
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};
use tracing::error;

#[derive(Debug)]
pub(crate) struct IconModule {
    /// Path to the module file. For example: src/my_module.rs.
    path: PathBuf,
}

impl IconModule {
    pub fn new<P: AsRef<Path>>(root: PathBuf, name: P) -> IconModule {
        let mut path = root.join(name.as_ref());
        path.set_extension("rs");
        if path.exists() {
            error!(?path, "Can not create IconModule. Path already exists");
        }
        IconModule { path }
    }

    pub async fn open_for_write(&mut self) -> Result<BufWriter<File>> {
        Ok(tokio::io::BufWriter::new(
            tokio::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(&self.path)
                .await
                .map_err(|err| {
                    error!(path = ?self.path, ?err, "Could not write to module file.");
                    err
                })?,
        ))
    }

    pub async fn write_components<'a, I: Iterator<Item = &'a [u8]>>(
        &mut self,
        components: I,
    ) -> Result<()> {
        let mut writer = self.open_for_write().await?;

        // TODO: Once https://github.com/leptos-rs/leptos/pull/748 is merged, this write can be removed. In component generation use `::leptos::...` wherever possible.
        writer
            .write_all("use leptos::*;\n\n".as_bytes())
            .await
            .unwrap();
        for bytes in components {
            writer.write_all(bytes).await.unwrap();
        }

        Ok(())
    }
}
