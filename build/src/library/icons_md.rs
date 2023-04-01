use anyhow::Result;
use std::{borrow::Cow, path::PathBuf};
use tokio::io::AsyncWriteExt;
use tracing::{error, instrument, trace};

use crate::{icon::SvgIcon, package::PackageType};

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
    async fn create_file(&self) -> Result<tokio::fs::File> {
        trace!("Creating file.");
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
    pub(crate) async fn reset(&self) -> Result<()> {
        if self.path.exists() {
            trace!("Removing file.");
            tokio::fs::remove_file(&self.path).await?;
        }

        trace!("Writing BASE_ICONS content.");
        self.create_file()
            .await?
            .write_all(BASE_ICONS.as_bytes())
            .await
            .map_err(Into::into)
    }

    #[instrument(level = "info")]
    async fn append(&self) -> Result<tokio::io::BufWriter<tokio::fs::File>> {
        trace!("Creating file.");
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
        package: PackageType,
        icons: &[SvgIcon],
    ) -> Result<()> {
        trace!("Writing icon table.");
        let mut file = self.append().await?;

        struct TableEntry<'a> {
            name: &'a str,
            categories: Cow<'a, str>,
        }

        let mut entries = Vec::new();
        entries.push(TableEntry {
            name: "Icon name",
            categories: "Categories".into(),
        });
        entries.push(TableEntry {
            name: "---",
            categories: "---".into(),
        });

        for icon in icons {
            entries.push(TableEntry {
                name: icon.feature.name.as_str(),
                categories: icon
                    .categories
                    .iter()
                    .map(|it| it.0.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
                    .into(),
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
            file.write_all(" ".repeat(longest_name - entry.name.len()).as_bytes())
                .await?;

            file.write_all(" | ".as_bytes()).await?;
            file.write_all(entry.categories.as_bytes()).await?;
            file.write_all(
                " ".repeat(longest_categories - entry.categories.len())
                    .as_bytes(),
            )
            .await?;

            file.write_all(" |".as_bytes()).await?;
            file.write_all("\n".as_bytes()).await?;
        }

        file.flush().await.map_err(|err| {
            error!(?err, "Could not flush file after writing.");
            err
        })?;

        Ok(())
    }
}
