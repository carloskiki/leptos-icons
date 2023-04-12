use std::path::PathBuf;

use anyhow::Result;
use tracing::{info, instrument, trace};

use crate::{icon_library::IconLibrary, readme_md::Readme};

use self::{cargo_toml::CargoToml, lib_rs::LibRs, src_dir::SrcDir};

mod cargo_toml;
mod lib_rs;
mod src_dir;

#[derive(Debug)]
pub(crate) struct MainLibrary {
    pub name: String,
    pub path: PathBuf,
    pub cargo_toml: CargoToml,
    pub readme_md: Readme<MainLibrary>,
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
                _phantom: std::marker::PhantomData,
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
        self.cargo_toml.reset().await?;
        self.cargo_toml.write_package_section(&self.name).await?;
        self.cargo_toml
            .write_dependencies_section(&icon_libs)
            .await?;
        self.cargo_toml.write_features_section(&icon_libs).await?;
        self.readme_md.reset().await?;

        self.src_dir
            .lib_rs
            .write(LibRs::build_reexports(&icon_libs)?)
            .await?;

        self.src_dir
            .lib_rs
            .write(LibRs::build_enum(&self.enum_name(), &icon_libs)?)
            .await?;

        self.src_dir
            .lib_rs
            .write(LibRs::build_component(
                &self.component_name(),
                &self.enum_name(),
                &icon_libs,
            )?)
            .await?;

        trace!("Writing README.md.");
        self.readme_md.write_readme().await?;

        info!("Library generated.");
        Ok(())
    }

    pub fn enum_name(&self) -> String {
        "Icon".to_owned()
    }

    pub fn component_name(&self) -> String {
        "LeptosIcon".to_owned()
    }
}
