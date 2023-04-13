use std::path::PathBuf;

use anyhow::Result;
use heck::ToUpperCamelCase;
use tracing::{error, info, instrument, trace};

use crate::{
    cargo_toml::CargoToml,
    icon::SvgIcon,
    icon_library::icons_md::Icons,
    package::{Downloaded, Package},
    readme_md::Readme, lib_rs::LibRs, src_dir::SrcDir,
};

mod icons_md;

#[derive(Debug)]
pub(crate) struct IconLibrary {
    pub package: Package<Downloaded>,
    pub name: String,
    pub path: PathBuf,
    pub cargo_toml: CargoToml<IconLibrary>,
    pub readme_md: Readme<IconLibrary>,
    pub src_dir: SrcDir<IconLibrary>,
    pub icons_md: Icons,
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
                _phantom: std::marker::PhantomData,
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
                    _phantom: std::marker::PhantomData,
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
        self.cargo_toml.reset().await?;
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

        self.src_dir.lib_rs.write_lib_rs(&self.enum_name(), &self.icons).await?;

        trace!("Writing crate manifest.");
        self.cargo_toml.write_cargo_toml(&self).await?;

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
