use std::path::PathBuf;

use anyhow::Result;
use tracing::{error, info, instrument};

use crate::{
    library::{
        cargo_toml::CargoToml, icons_md::Icons, lib_rs::LibRs, readme_md::Readme, src_dir::SrcDir,
    },
    package::{Downloaded, Package},
};

mod cargo_toml;
mod icons_md;
mod lib_rs;
mod readme_md;
mod src_dir;

#[derive(Debug)]
pub(crate) struct Library {
    package: Package<Downloaded>,
    name: String,
    path: PathBuf,
    cargo_toml: CargoToml,
    readme_md: Readme,
    icons_md: Icons,
    src_dir: SrcDir,
}

impl Library {
    pub fn new(package: Package<Downloaded>, name: String, root: PathBuf) -> Self {
        Self {
            package,
            name,
            path: root.clone(),
            cargo_toml: CargoToml {
                path: root.join("Cargo.toml"),
            },
            readme_md: Readme {
                path: root.join("README.md"),
            },
            icons_md: Icons {
                path: root.join("ICONS.md"),
            },
            src_dir: SrcDir {
                path: root.join("src"),
                lib_rs: LibRs {
                    path: root.join("src").join("lib.rs"),
                },
            },
        }
    }

    #[instrument(level = "info", skip(self), fields(package = ?self.package.ty))]
    pub async fn generate(&mut self) -> Result<()> {
        info!("Ensuring library directory exists.");
        if !self.path.exists() {
            tokio::fs::create_dir_all(&self.path).await?;
        }

        info!("Resetting library directory.");
        self.src_dir.reset().await?;
        self.cargo_toml.reset(&self.name).await?;
        self.readme_md.reset().await?;
        self.icons_md.reset().await?;

        // Extract icon information from that package.
        // Sorting the resulting Vec is necessary, as we want to reduce churn in the later generated output as much as possible.
        info!("Collecting icons.");
        let mut icons = self.package.read_icons().await.map_err(|err| {
            error!(?err, "Could not get icons.");
            err
        })?;

        info!(num_icons = icons.len(), "Sorting icons to avoid churn.");
        icons.sort_by(|a, b| a.feature.name.cmp(&b.feature.name));

        self.src_dir.lib_rs.write_enum(&icons).await?;
        self.src_dir
            .lib_rs
            .write_leptos_icon_component(&icons)
            .await?;
        self.cargo_toml.append_features(&icons).await?;

        info!("Writing README.md.");
        self.readme_md.write_usage().await?;
        self.readme_md.write_package_table().await?;
        self.readme_md.write_contribution().await?;

        info!("Writing ICONS.md.");
        self.icons_md
            .write_icon_table(self.package.ty, &icons)
            .await?;

        Ok(())
    }
}
