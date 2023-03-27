use std::path::{Path, PathBuf};

use crate::path;

use self::{
    cargo_toml::CargoToml, icons_md::Icons, lib_rs::LibRs, readme_md::Readme, src_dir::SrcDir,
};

pub(crate) mod cargo_toml;
pub(crate) mod icons_md;
pub(crate) mod lib_rs;
pub(crate) mod readme_md;
pub(crate) mod src_dir;

#[derive(Debug)]
pub(crate) struct Library {
    path: PathBuf,
    cargo_toml: CargoToml,
    readme_md: Readme,
    icons_md: Icons,
    src_dir: SrcDir,
}

impl Library {
    pub fn new() -> Self {
        Self {
            path: path::leptos_icons_crate(""),
            cargo_toml: CargoToml {
                path: path::leptos_icons_crate("Cargo.toml"),
            },
            readme_md: Readme {
                path: path::leptos_icons_crate("README.md"),
            },
            icons_md: Icons {
                path: path::leptos_icons_crate("ICONS.md"),
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

    pub fn readme_md(&self) -> &Readme {
        &self.readme_md
    }

    pub fn icons_md(&self) -> &Icons {
        &self.icons_md
    }

    pub fn src_dir(&self) -> &SrcDir {
        &self.src_dir
    }

    #[allow(unused)]
    pub fn relative_path<P: AsRef<Path>>(&self, join: P) -> PathBuf {
        self.path.join(join)
    }
}
