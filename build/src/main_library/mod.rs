use std::path::PathBuf;

use anyhow::Result;
use tracing::{info, instrument, trace};

use crate::library::IconLibrary;

use self::{cargo_toml::CargoToml, lib_rs::LibRs, readme_md::Readme, src_dir::SrcDir};

mod cargo_toml;
mod lib_rs;
mod readme_md;
mod src_dir;

#[derive(Debug)]
pub(crate) struct MainLibrary {
    pub name: String,
    pub path: PathBuf,
    pub cargo_toml: CargoToml,
    pub readme_md: Readme,
    pub src_dir: SrcDir,
}

impl MainLibrary {
    pub fn new(name: String, root: PathBuf) -> Self {
        Self {
            name,
            path: root.clone(),
            cargo_toml: CargoToml {
                path: root.join("Cargo.toml"),
            },
            readme_md: Readme {
                path: root.join("README.md"),
            },
            src_dir: SrcDir {
                path: root.join("src"),
                lib_rs: LibRs {
                    path: root.join("src").join("lib.rs"),
                },
            },
        }
    }

    #[instrument(level = "info", skip_all)]
    pub async fn generate(&mut self, icon_libs: Vec<IconLibrary>) -> Result<()> {
        trace!("Ensuring library directory exists.");
        if !self.path.exists() {
            tokio::fs::create_dir_all(&self.path).await?;
        }

        trace!("Resetting library directory.");
        self.src_dir.reset().await?;
        self.cargo_toml.reset(&self.name).await?;
        self.readme_md.reset().await?;

        let enum_code = LibRs::build_enum(&self.enum_name(), &icon_libs)?;
        self.src_dir.lib_rs.write_enum(enum_code).await?;

        self.cargo_toml.append_dependencies(&icon_libs).await?;

        trace!("Writing README.md.");
        self.readme_md.write_usage().await?;
        self.readme_md.write_package_table().await?;
        self.readme_md.write_contribution().await?;

        info!("Library generated.");
        Ok(())
    }

    pub fn enum_name(&self) -> String {
        "Icon".to_owned()
    }
}
