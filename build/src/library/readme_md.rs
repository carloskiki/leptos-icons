use anyhow::Result;
use indoc::indoc;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use tracing::{error, info, instrument};

use crate::package::{GitTarget, Package, PackageSource};

const BASE_README: &str = indoc::indoc!(
    r#"
    # Leptos-Icons

    Add icons from popular icon libraries into your leptos projects. Every icon is packaged as its own cargo feature to reduce build times.

    - Please note that this crate is in very early developpement and may include [bugs](#contributing).
    - This crate is **heavily** inspired by the [solidjs-icons](https://github.com/x64Bits/solid-icons) library.

    ## Table of Contents

    - [Leptos-Icons](#leptos-icons)
      - [Table of Contents](#table-of-contents)
      - [Usage](#usage)
      - [Icon Packages](#icon-packages)
      - [Contributing](#contributing)

"#
);

#[derive(Debug)]
pub(crate) struct Readme {
    pub path: PathBuf,
}

impl Readme {
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
                error!(?err, "Could not create readme.");
                err
            })
            .map_err(Into::into)
    }

    #[instrument(level = "info")]
    pub async fn init(&self) -> Result<()> {
        info!("Writing BASE_README content.");
        self.create_file()
            .await?
            .write_all(BASE_README.as_bytes())
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

    pub async fn write_usage(&self) -> Result<()> {
        info!("Writing usage section.");
        let section = indoc::indoc! {r#"
            ## Usage

            To use this crate, it is currently required to use Git linking, as it is not yet published to crates.io.
            Use icons by specifying their feature names. For example `BsFolder` for the Bootstrap-Icons `Folder` icon.

            ```toml
            [dependencies]
            # ...
            leptos-icons = { git = "https://github.com/Carlosted/leptos-icons.git" features = ["BsFolder"] }
            ```

        "#};
        let mut file = self.append().await?;
        file.write_all(section.as_bytes()).await?;
        file.flush().await.map_err(|err| {
            error!(?err, "Could not flush file after writing.");
            err
        })?;
        Ok(())
    }

    pub async fn write_contribution(&self) -> Result<()> {
        info!("Writing contribution section.");
        let section = indoc::indoc! {r#"
            ## Contributing

            Contributions are more than welcomed!
            Do not hesitate add icon libraries, features, etc.
        "#};
        let mut file = self.append().await?;
        file.write_all(section.as_bytes()).await?;
        file.flush().await.map_err(|err| {
            error!(?err, "Could not flush file after writing.");
            err
        })?;
        Ok(())
    }

    pub async fn write_package_table(&self) -> Result<()> {
        info!("Writing package table.");

        struct TableEntry {
            name: String,
            version: String,
            source: String,
            license: String,
            short_name: String,
        }

        let mut entries = Vec::new();
        entries.push(TableEntry {
            name: "Icon Library".to_owned(),
            version: "Version".to_owned(),
            source: "Source".to_owned(),
            license: "License".to_owned(),
            short_name: "Short name".to_owned(),
        });
        entries.push(TableEntry {
            name: "---".to_owned(),
            version: "---".to_owned(),
            source: "---".to_owned(),
            license: "---".to_owned(),
            short_name: "---".to_owned(),
        });

        for package in Package::all() {
            entries.push(TableEntry {
                name: package.meta.package_name.clone().into_owned(),
                version: match &package.meta.source {
                    PackageSource::Git { url: _, target } => match &target {
                        GitTarget::Branch {
                            name: _,
                            commit_ref: _,
                            version_hint,
                        } => version_hint
                            .clone()
                            .map(|it| it.to_string())
                            .unwrap_or("unknown".to_owned()),
                        GitTarget::Tag { name: _, version } => {
                            format!("{version}")
                        }
                    },
                },
                source: match &package.meta.source {
                    PackageSource::Git { url, target } => match &target {
                        GitTarget::Branch {
                            name,
                            commit_ref,
                            version_hint: _,
                        } => format!("Git: <{url}> - Branch: {name} - Commit: {commit_ref}"),
                        GitTarget::Tag { name, version: _ } => {
                            format!("Git: <{url}> - Tag: {name}")
                        }
                    },
                },
                license: package
                    .meta
                    .licenses
                    .iter()
                    .fold(String::new(), |mut acc, s| {
                        acc.push_str(s);
                        acc.push_str(", ");
                        acc
                    }),
                short_name: package.meta.short_name.clone().into_owned(),
            });
        }

        let section_header = indoc! { r#"
            ## Icon Packages

            Licenses of the icons provided through these libraries were extracted with best intent,
            but must only be taken as a hint. Please check the individual icon repositories for up-to-date license information.

        "#};

        let mut file = self.append().await?;
        file.write_all(section_header.as_bytes()).await?;

        let longest_name = entries
            .iter()
            .fold(0, |acc, it| usize::max(acc, it.name.len()));
        let longest_version = entries
            .iter()
            .fold(0, |acc, it| usize::max(acc, it.version.len()));
        let longest_source = entries
            .iter()
            .fold(0, |acc, it| usize::max(acc, it.source.len()));
        let longest_license = entries
            .iter()
            .fold(0, |acc, it| usize::max(acc, it.license.len()));
        let longest_short_name = entries
            .iter()
            .fold(0, |acc, it| usize::max(acc, it.short_name.len()));

        for entry in entries {
            file.write_all("| ".as_bytes()).await?;
            file.write_all(entry.name.as_bytes()).await?;
            file.write_all(" ".repeat(longest_name - entry.name.len()).as_bytes())
                .await?;

            file.write_all(" | ".as_bytes()).await?;
            file.write_all(entry.version.as_bytes()).await?;
            file.write_all(" ".repeat(longest_version - entry.version.len()).as_bytes())
                .await?;

            file.write_all(" | ".as_bytes()).await?;
            file.write_all(entry.source.as_bytes()).await?;
            file.write_all(" ".repeat(longest_source - entry.source.len()).as_bytes())
                .await?;

            file.write_all(" | ".as_bytes()).await?;
            file.write_all(entry.license.as_bytes()).await?;
            file.write_all(" ".repeat(longest_license - entry.license.len()).as_bytes())
                .await?;

            file.write_all(" | ".as_bytes()).await?;
            file.write_all(entry.short_name.as_bytes()).await?;
            file.write_all(
                " ".repeat(longest_short_name - entry.short_name.len())
                    .as_bytes(),
            )
            .await?;

            file.write_all(" |".as_bytes()).await?;
            file.write_all("\n".as_bytes()).await?;
        }
        file.write_all("\n".as_bytes()).await?;

        file.flush().await.map_err(|err| {
            error!(?err, "Could not flush file after writing.");
            err
        })?;

        Ok(())
    }
}
