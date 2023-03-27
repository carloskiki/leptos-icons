use std::path::{Path, PathBuf};

use crate::path;

use self::{cargo_toml::CargoToml, lib_rs::LibRs, src_dir::SrcDir};

pub(crate) mod cargo_toml;
pub(crate) mod lib_rs;
pub(crate) mod src_dir;

#[derive(Debug)]
pub(crate) struct Library {
    path: PathBuf,
    cargo_toml: CargoToml,
    src_dir: SrcDir,
}

impl Library {
    pub fn new() -> Self {
        Self {
            path: path::leptos_icons_crate(""),
            cargo_toml: CargoToml {
                file_path: path::leptos_icons_crate("Cargo.toml"),
            },
            src_dir: SrcDir {
                path: path::leptos_icons_crate("src"),
                lib_rs: LibRs {
                    path: path::leptos_icons_crate("src").join("lib.rs"),
                },
            },
        }
    }

    pub fn cargo_toml(&self) -> &CargoToml {
        &self.cargo_toml
    }

    pub fn src_dir(&self) -> &SrcDir {
        &self.src_dir
    }

    #[allow(unused)]
    pub fn relative_path<P: AsRef<Path>>(&self, join: P) -> PathBuf {
        self.path.join(join)
    }
}
