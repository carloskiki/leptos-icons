use std::path::PathBuf;

use anyhow::Result;
use heck::ToUpperCamelCase;
use tracing::{error, info, instrument, trace};

use crate::{
    icon::SvgIcon,
    icon_library::{
        cargo_toml::CargoToml, icons_md::Icons, lib_rs::LibRs, src_dir::SrcDir,
    },
    package::{Downloaded, Package}, readme_md::Readme,
};

mod cargo_toml;
mod icons_md;
mod lib_rs;
mod src_dir;

#[derive(Debug)]
pub(crate) struct IconLibrary {
    pub package: Package<Downloaded>,
    pub name: String,
    pub path: PathBuf,
    pub cargo_toml: CargoToml,
    pub readme_md: Readme<IconLibrary>,
    pub icons_md: Icons,
    pub src_dir: SrcDir,
    pub icons: Vec<SvgIcon>,
}

impl IconLibrary {
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
                _phantom: std::marker::PhantomData,
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
            icons: Vec::new(),
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
        self.cargo_toml.reset(&self.name, &self.package.meta.package_name).await?;
        self.readme_md.reset().await?;
        self.icons_md.reset().await?;

        // Extract icon information from that package.
        // Sorting the resulting Vec is necessary, as we want to reduce churn in the later generated output as much as possible.
        trace!("Collecting icons.");
        self.icons = self.package.read_icons().await.map_err(|err| {
            error!(?err, "Could not get icons.");
            err
        })?;

        trace!(
            num_icons = self.icons.len(),
            "Sorting icons to avoid churn."
        );
        self.icons
            .sort_by(|a, b| a.feature.name.cmp(&b.feature.name));

        let enum_code = LibRs::build_enum(&self.enum_name(), &self.icons)?;
        self.src_dir.lib_rs.write_enum(enum_code).await?;

        let component_code = LibRs::build_icon_component(&self.enum_name(), &self.icons)?;
        self.src_dir.lib_rs.write_component(component_code).await?;

        self.cargo_toml.append_features(&self.icons).await?;

        trace!("Writing README.md.");
        self.readme_md.write_readme(&self.package.meta).await?;

        trace!("Writing ICONS.md.");
        self.icons_md
            .write_icon_table(self.package.ty, &self.icons)
            .await?;

        info!("Library generated.");
        Ok(())
    }

    pub fn enum_name(&self) -> String {
        format!("{}Icon", self.package.meta.short_name.to_upper_camel_case())
    }
}
