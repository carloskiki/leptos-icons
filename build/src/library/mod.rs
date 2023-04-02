use std::path::PathBuf;

use anyhow::Result;
use heck::ToUpperCamelCase;
use tracing::{error, info, instrument, trace};

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
    pub package: Package<Downloaded>,
    pub name: String,
    pub path: PathBuf,
    pub cargo_toml: CargoToml,
    pub readme_md: Readme,
    pub icons_md: Icons,
    pub src_dir: SrcDir,
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
        trace!("Ensuring library directory exists.");
        if !self.path.exists() {
            tokio::fs::create_dir_all(&self.path).await?;
        }

        trace!("Resetting library directory.");
        self.src_dir.reset().await?;
        self.cargo_toml.reset(&self.name).await?;
        self.readme_md.reset().await?;
        self.icons_md.reset().await?;

        // Extract icon information from that package.
        // Sorting the resulting Vec is necessary, as we want to reduce churn in the later generated output as much as possible.
        trace!("Collecting icons.");
        let mut icons = self.package.read_icons().await.map_err(|err| {
            error!(?err, "Could not get icons.");
            err
        })?;
        trace!(num_icons = icons.len(), "Sorting icons to avoid churn.");
        icons.sort_by(|a, b| a.feature.name.cmp(&b.feature.name));

        let enum_code = LibRs::build_enum(&self.enum_name(), &icons)?;
        self.src_dir.lib_rs.write_enum(enum_code).await?;

        let component_code =
            LibRs::build_icon_component(&self.component_name(), &self.enum_name(), &icons)?;
        self.src_dir.lib_rs.write_component(component_code).await?;

        self.cargo_toml.append_features(&icons).await?;

        trace!("Writing README.md.");
        self.readme_md.write_usage().await?;
        self.readme_md.write_package_table().await?;
        self.readme_md.write_contribution().await?;

        trace!("Writing ICONS.md.");
        self.icons_md
            .write_icon_table(self.package.ty, &icons)
            .await?;

        info!("Library generated.");
        Ok(())
    }

    pub fn component_name(&self) -> String {
        format!(
            "Leptos{}Icon",
            self.package.meta.short_name.to_upper_camel_case()
        )
    }

    pub fn enum_name(&self) -> String {
        format!("{}Icon", self.package.meta.short_name.to_upper_camel_case())
    }
}
