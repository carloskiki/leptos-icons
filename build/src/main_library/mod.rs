use std::path::PathBuf;

use anyhow::Result;
use tracing::{info, instrument, trace};

use crate::{cargo_toml::CargoToml, icon_library::IconLibrary, lib_rs::LibRs, readme_md::Readme};

use crate::src_dir::SrcDir;

#[derive(Debug)]
pub(crate) struct MainLibrary {
    pub path: PathBuf,
    pub cargo_toml: CargoToml<MainLibrary>,
    pub readme_md: Readme<MainLibrary>,
    pub src_dir: SrcDir<MainLibrary>,
}

impl MainLibrary {
    pub fn new(root: PathBuf) -> Self {
        Self {
            path: root.clone(),
            cargo_toml: CargoToml {
                path: root.join("Cargo.toml"),
                _phantom: std::marker::PhantomData,
            },
            readme_md: Readme {
                path: root.join("README.md"),
                _phantom: std::marker::PhantomData,
            },
            src_dir: SrcDir {
                path: root.join("src"),
                lib_rs: LibRs {
                    path: root.join("src").join("lib.rs"),
                    _phantom: std::marker::PhantomData,
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
        self.cargo_toml.write_cargo_toml(&icon_libs).await?;
        self.readme_md.reset().await?;

        self.src_dir
            .lib_rs
            .write_lib_rs(&self.component_name(), &self.enum_name(), &icon_libs).await?;

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
