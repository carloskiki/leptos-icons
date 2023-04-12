use anyhow::Result;
use indoc::{formatdoc, indoc};
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use tracing::{error, instrument, trace};

use crate::{package::{GitTarget, Package, PackageMetadata, PackageSource}, main_library::MainLibrary, icon_library::IconLibrary};

#[derive(Debug)]
pub(crate) struct Readme<T> {
    pub(crate) path: PathBuf,
    pub(crate) _phantom: std::marker::PhantomData<T>,
}

#[derive(Debug)]
pub(crate) struct BaseRepo;

impl<T: std::fmt::Debug> Readme<T> {
    #[instrument(level = "info")]
    async fn create_file(&self) -> Result<tokio::fs::File> {
        trace!("Creating file.");
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
    pub async fn reset(&self) -> Result<()> {
        if self.path.exists() {
            trace!("Removing file.");
            tokio::fs::remove_file(&self.path).await?;
        }

        trace!("Writing BASE_README content.");
        self.create_file().await?;

        Ok(())
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

    async fn write_section(&self, section: &str) -> Result<()> {
        let mut file = self.append().await?;
        file.write_all(section.as_bytes()).await?;
        file.flush().await.map_err(|err| {
            error!(?err, "Could not flush file after writing.");
            err
        })?;
        Ok(())
    }

    async fn write_contribution(&self) -> Result<()> {
        trace!("Writing contribution section.");
        let contribution = indoc::indoc! {r#"
            ## Contributing

            Contributions are more than welcomed!
            Do not hesitate add icon libraries, features, etc.
        "#};

        self.write_section(contribution).await
    }
}

impl Readme<MainLibrary> {
    pub(crate) async fn write_readme(&self) -> Result<()> {
        self.write_header().await?;
        self.write_usage().await?;
        self.write_package_table().await?;
        self.write_contribution().await?;

        Ok(())
    }
    async fn write_header(&self) -> Result<()> {
        trace!("Writing header section.");
        let header = indoc! {r#"
            # Leptos-Icons

            Add icons from popular icon libraries into your leptos projects. Every icon is packaged as its own cargo feature to reduce build times.

            - This crate is inspired by other frameworks' icon libraries such as [solidjs-icons](https://github.com/x64Bits/solid-icons),
            [yew_icons](https://github.com/finnbear/yew_icons),
            [dioxus-free-icons](https://github.com/nissy-dev/dioxus-free-icons),
            etc.

            ## Table of Contents

            - [Leptos-Icons](#leptos-icons)
            - [Table of Contents](#table-of-contents)
            - [Usage](#usage)
            - [Icon Packages](#icon-packages)
            - [Contributing](#contributing)

            "#};

        self.write_section(header).await
    }

    async fn write_usage(&self) -> Result<()> {
        trace!("Writing usage section.");
        let usage = indoc::indoc! {r#"
            ## Usage

            To use this crate, it is currently required to use Git linking, as it is not yet published to crates.io.
            Use icons by specifying their feature names. For example `BsFolder` for the Bootstrap-Icons `Folder` icon.

            ```toml
            [dependencies]
            # ...
            leptos-icons = { git = "https://github.com/Carlosted/leptos-icons.git" features = ["BsFolder"] }
            ```

        "#};

        self.write_section(usage).await
    }

    async fn write_package_table(&self) -> Result<()> {
        trace!("Writing package table.");

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

impl Readme<IconLibrary> {
    pub(crate) async fn write_readme(&self, package_meta: &PackageMetadata) -> Result<()> {
        self.write_header(package_meta).await?;
        self.write_usage(package_meta).await?;
        self.write_contribution().await?;

        Ok(())
    }

    async fn write_header(&self, package_meta: &PackageMetadata) -> Result<()> {
        trace!("Writing header section.");
        let short_name = &package_meta.short_name;
        let header = formatdoc!(
            r#"
            # Leptos-Icons-{short_name}

            Add icons from the {} library into your leptos projects. Every icon is packaged as its own cargo feature to reduce build times.

            ## Table of Contents

            - [Leptos-Icons-{short_name}](#leptos-icons-{short_name})
              - [Table of Contents](#table-of-contents)
              - [Usage](#usage)
              - [Contributing](#contributing)

            "#,
            package_meta.package_name
        );

        self.write_section(&header).await
    }

    async fn write_usage(&self, package_meta: &PackageMetadata) -> Result<()> {
        trace!("Writing usage section.");
        let usage = formatdoc!(
            r#"
            ## Usage

            To use this crate, it is currently required to use Git linking, as it is not published to crates.io.
            Use icons by specifying their feature names. It is recommended to use the main crate `leptos-icons` instead.

            ```toml
            [dependencies]
            # ...
            leptos-icons-{} = {{ git = "https://github.com/Carlosted/leptos-icons.git" features = ["..."] }}
            ```

        "#,
            package_meta.short_name
        );

        self.write_section(&usage).await
    }
}

impl Readme<BaseRepo> {
    pub async fn write_readme(&self) -> Result<()> {
        self.write_header().await?;
        self.write_repository_content().await?;
        self.write_developing().await?;
        self.write_contribution().await?;

        Ok(())
    }

    async fn write_header(&self) -> Result<()> {
        trace!("Writing header section.");
        let header = indoc! {r#"
            # Leptos-Icons

            This repository is the parent for the `leptos-icons` crate.
            It also contains the build crate, the leptos-icons-core crate, and crates for individual icon packages.

            ## Table of Contents
            - [Leptos-Icons](#leptos-icons)
            - [Table of Contents](#table-of-contents)
            - [Repository Content](#repository-content)
            - [Developing](#developing)
            - [Contributing](#contributing)
            "#};

        self.write_section(header).await
    }

    async fn write_developing(&self) -> Result<()> {
        let developing = indoc! {r#"
            ## Developing

            This repository uses Just

            Simply call
            ```bash
            just
            ```
            to see a list of available commands.

            You may need to install just using

            ```bash
            cargo install just
            ```

            "#};

        self.write_section(developing).await
    }

    async fn write_repository_content(&self) -> Result<()> {
        trace!("Writing the repository content section header.");
        let section_header = indoc! {r#"
            ## Repository Content

            here is what the repository is parent for:

            "#};

        struct TableEntry {
            lib: String,
            description: String,
        }

        let mut entries = Vec::new();
        entries.push(TableEntry {
            lib: "Crate".to_owned(),
            description: "Description".to_owned(),
        });
        entries.push(TableEntry {
            lib: "---".to_owned(),
            description: "---".to_owned(),
        });

        entries.push(TableEntry {
            lib: "leptos-icons".to_owned(),
            description: "The main icon library; the whole point of this repository.".to_owned(),
        });
        entries.push(TableEntry {
            lib: "leptos-icons-core".to_owned(),
            description: "A core library that contains utilities for the leptos-icons crate."
                .to_owned(),
        });
        entries.push(TableEntry {
            lib: "build".to_owned(),
            description: "The build crate that generates most of this repository.".to_owned(),
        });
        for package in Package::all() {
            entries.push(TableEntry {
                lib: format!("leptos-icons-{}", package.meta.short_name),
                description: format!("The icon library for {}.", package.meta.package_name),
            })
        }

        let mut file = self.append().await?;
        file.write_all(section_header.as_bytes()).await?;

        for entry in entries {
            file.write_all("| ".as_bytes()).await?;
            file.write_all(entry.lib.as_bytes()).await?;
            file.write_all(" | ".as_bytes()).await?;
            file.write_all(entry.description.as_bytes()).await?;
            file.write_all(" |\n".as_bytes()).await?;
        }

        file.flush().await.map_err(|err| {
            error!(?err, "Could not flush file after writing.");
            err
        })?;

        Ok(())
    }
}
