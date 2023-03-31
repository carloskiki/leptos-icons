use std::{path::PathBuf, sync::Arc, vec};

use anyhow::Result;
use tokio::sync::RwLock;
use tracing::{error, info};

use crate::{
    icon::{IconMetadata, SvgIcon},
    library::{
        cargo_toml::CargoToml, icons_md::Icons, lib_rs::LibRs, readme_md::Readme, src_dir::SrcDir,
    },
    package::{Package, PackageType},
};

mod cargo_toml;
mod icons_md;
mod lib_rs;
mod readme_md;
mod src_dir;

#[derive(Debug)]
pub(crate) struct Library {
    #[allow(unused)]
    path: PathBuf,
    cargo_toml: CargoToml,
    readme_md: Readme,
    icons_md: Icons,
    src_dir: SrcDir,
}

impl Library {
    pub fn new(root: PathBuf) -> Self {
        Self {
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

    pub async fn generate(mut self, clean: bool) -> Result<()> {
        info!("Resetting library directory.");
        self.src_dir.reset().await?;
        self.cargo_toml.remove().await?;
        self.cargo_toml.init().await?;
        self.readme_md.remove().await?;
        self.readme_md.init().await?;
        self.icons_md.remove().await?;
        self.icons_md.init().await?;

        let all_icons = Arc::new(RwLock::new(Vec::<SvgIcon>::new()));

        let handles = Package::all()
            .into_iter()
            .map(|package| {
                let package_type = package.ty;
                let all_icons = all_icons.clone();
                tokio::spawn(async move {
                    if clean {
                        package.remove().await?;
                    }

                    // Download the package.
                    let package = package.download().map_err(|err| {
                        error!(
                            ?package_type,
                            ?err,
                            "Downloading the package failed unexpectedly."
                        );
                        err
                    })?;

                    // Extract icon information from that package.
                    // Sorting the resulting Vec is necessary, as we want to reduce churn in the later generated output as much as possible.
                    let mut icons = package.read_icons().await.map_err(|err| {
                        error!(?package_type, ?err, "Could not get icons.");
                        err
                    })?;

                    info!(?package_type, "Collecting icons.");
                    {
                        let mut lock = all_icons.write().await;
                        lock.append(&mut icons);
                    }

                    Ok::<(), anyhow::Error>(())
                })
            })
            .collect::<Vec<_>>();
        for handle in handles {
            if let Err(err) = handle.await.unwrap() {
                error!(?err, "Could not process package successfully.");
            }
        }

        let all_icons = {
            let mut lock = all_icons.write().await;
            let num_features = lock.len();
            info!(num_features, "Sorting features to avoid churn.");
            lock.sort_by(|a, b| a.feature.name.cmp(&b.feature.name));
            std::mem::take(&mut *lock)
        };
        self.src_dir.lib_rs.write_enum(&all_icons).await?;
        self.src_dir
            .lib_rs
            .write_leptos_icon_component(&all_icons)
            .await?;
        self.cargo_toml.append_features(&all_icons).await?;

        info!("Writing README.md.");
        self.readme_md.write_usage().await?;
        self.readme_md.write_package_table().await?;
        self.readme_md.write_contribution().await?;

        info!("Writing ICONS.md.");
        let mut package_icon_metadata: Vec<(PackageType, Vec<IconMetadata>)> =
            Package::all().into_iter().map(|p| (p.ty, vec![])).collect();
        for package in Package::all() {
            let meta = all_icons
                .iter()
                .filter(|icon| icon.source == package.ty)
                .map(|icon| IconMetadata {
                    name: icon.feature.name.clone(),
                    categories: icon.categories.clone(),
                })
                .collect::<Vec<_>>();

            package_icon_metadata
                .iter_mut()
                .find(|(p, _vec)| *p == package.ty)
                .expect("should have been initialized")
                .1 = meta;
        }
        self.icons_md
            .write_icon_table(package_icon_metadata)
            .await?;

        Ok(())
    }
}
