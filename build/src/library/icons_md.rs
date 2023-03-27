use anyhow::Result;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use tracing::{error, info, instrument};

use crate::{icon::IconMeta, package::PackageType};

const BASE_ICONS: &str = indoc::indoc!(
    r#"
    # Icons
"#
);

#[derive(Debug)]
pub(crate) struct Icons {
    pub path: PathBuf,
}

impl Icons {
    #[instrument(level = "info")]
    pub async fn remove(&self) -> Result<()> {
        if self.path.exists() {
            info!("Removing file.");
            tokio::fs::remove_file(&self.path).await.map_err(Into::into)
        } else {
            Ok(())
        }
    }

    #[instrument(level = "info")]
    async fn create_file(&self) -> Result<tokio::fs::File> {
        info!("Creating file.");
        tokio::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&self.path)
            .await
            .map_err(|err| {
                error!(?err, "Could not create icons.");
                err
            })
            .map_err(Into::into)
    }

    #[instrument(level = "info")]
    pub(crate) async fn init(&self) -> Result<()> {
        info!("Writing BASE_ICONS content.");
        self.create_file()
            .await?
            .write_all(BASE_ICONS.as_bytes())
            .await
            .map_err(Into::into)
    }

    #[instrument(level = "info")]
    async fn append(&self) -> Result<tokio::io::BufWriter<tokio::fs::File>> {
        info!("Creating file.");
        Ok(tokio::io::BufWriter::new(
            tokio::fs::OpenOptions::new()
                .append(true)
                .open(&self.path)
                .await
                .map_err(|err| {
                    error!(?err, "Could not open file to append data.");
                    err
                })?,
        ))
    }

    pub(crate) async fn write_icon_table(
        &self,
        package_icon_metadata: Vec<(PackageType, Vec<IconMeta>)>,
    ) -> Result<()> {
        info!("Writing icon table.");

        let mut file = self.append().await?;

        struct TableEntry {
            name: String,
            categories: String,
        }

        for (package, icon_metadata) in package_icon_metadata {
            let mut entries = Vec::new();
            entries.push(TableEntry {
                name: "Icon name".to_owned(),
                categories: "Categories".to_owned(),
            });
            entries.push(TableEntry {
                name: "---".to_owned(),
                categories: "---".to_owned(),
            });

            for icon_meta in icon_metadata {
                entries.push(TableEntry {
                    name: icon_meta.name,
                    categories: icon_meta
                        .categories
                        .into_iter()
                        .map(|it| it.0)
                        .collect::<Vec<_>>()
                        .join(", "),
                });
            }

            file.write_all(format!("\n## {package:?}\n\n").as_bytes())
                .await?;

            let longest_name = entries
                .iter()
                .fold(0, |acc, it| usize::max(acc, it.name.len()));
            let longest_categories = entries
                .iter()
                .fold(0, |acc, it| usize::max(acc, it.categories.len()));

            for entry in entries {
                file.write_all("| ".as_bytes()).await?;
                file.write_all(entry.name.as_bytes()).await?;
                file.write_all(
                    std::iter::repeat(' ')
                        .take(longest_name - entry.name.len())
                        .collect::<String>()
                        .as_bytes(),
                )
                .await?;

                file.write_all(" | ".as_bytes()).await?;
                file.write_all(entry.categories.as_bytes()).await?;
                file.write_all(
                    std::iter::repeat(' ')
                        .take(longest_categories - entry.categories.len())
                        .collect::<String>()
                        .as_bytes(),
                )
                .await?;

                file.write_all(" |".as_bytes()).await?;
                file.write_all("\n".as_bytes()).await?;
            }
        }

        file.flush().await.map_err(|err| {
            error!(?err, "Could not flush file after writing.");
            err
        })?;

        Ok(())
    }
}
