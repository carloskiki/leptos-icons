use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use strum::IntoEnumIterator;
use tokio::{io::AsyncWriteExt, sync::RwLock};
use tracing::{error, info};

use crate::{
    feature::Feature,
    icon::IconMetadata,
    library::{
        cargo_toml::CargoToml, icon_module::IconModule, icons_md::Icons, lib_rs::LibRs,
        readme_md::Readme, src_dir::SrcDir,
    },
    package::{Package, PackageType},
};

mod cargo_toml;
mod icon_module;
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
                icon_modules: Vec::new(),
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

        let features = Arc::new(RwLock::new(Vec::<Feature>::new()));
        let modules = Arc::new(RwLock::new(Vec::<String>::new()));
        let package_icon_metadata = Arc::new(RwLock::new(
            PackageType::iter().map(|p| (p, vec![])).collect::<Vec<_>>(),
        ));
        let src_root = self.src_dir.path.clone();
        let src_dir: Arc<RwLock<SrcDir>> = Arc::new(RwLock::new(self.src_dir));

        let handles = Package::all()
            .into_iter()
            .map(|package| {
                let package_type = package.ty;
                let features = features.clone();
                let modules = modules.clone();
                let package_icon_metadata = package_icon_metadata.clone();
                let src_root = src_root.clone();
                let src_dir = src_dir.clone();
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
                    icons.sort_by(|icon_a, icon_b| icon_a.feature.name.cmp(&icon_b.feature.name));

                    info!(?package_type, "Collecting icon metadata.");
                    {
                        let meta = icons
                            .iter()
                            .map(|icon| IconMetadata {
                                name: icon.feature.name.clone(),
                                categories: icon.categories.clone(),
                            })
                            .collect::<Vec<_>>();

                        let mut lock = package_icon_metadata.write().await;
                        lock.iter_mut()
                            .find(|(p, _vec)| *p == package.ty)
                            .expect("should have been initialized")
                            .1 = meta;
                    }

                    info!(?package_type, "Collecting feature names.");
                    {
                        let mut lock = features.write().await;
                        for icon in &icons {
                            lock.push(icon.feature.clone());
                        }
                    }

                    info!(?package_type, "Collecting module name.");
                    {
                        let mut lock = modules.write().await;
                        lock.push(package.meta.short_name.clone().into_owned());
                    }

                    // Generate leptos icon components. Note that these sorted correctly, as the icons were already sorted.
                    info!(?package_type, "Generating leptos icon components.");
                    let icon_components = icons
                        .into_iter()
                        .map(|icon| {
                            icon.create_leptos_icon_component().unwrap() // TODO:: Error handling
                        })
                        .collect::<Vec<_>>();

                    // Writing leptos icon components.
                    info!(
                        ?package_type,
                        num_components = icon_components.len(),
                        "Writing leptos icon components into module."
                    );
                    let mut module = IconModule::new(src_root, package.meta.short_name.as_ref());
                    module
                        .write_components(icon_components.iter().map(|it| it.0.as_bytes()))
                        .await?;
                    {
                        let mut lock = src_dir.write().await;
                        lock.add_module(module);
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

        // Unpack src_dir, as it is no longer in shared access at this point...
        self.src_dir = Arc::try_unwrap(src_dir).expect("single owner").into_inner();

        let mut modules = modules.write().await;
        let num_modules = modules.len();

        info!(num_modules, "Sorting modules to avoid churn.");
        modules.sort();

        info!(num_modules, "Writing modules to lib.rs.");
        let mut lib_rs = self.src_dir.lib_rs().append().await?;
        for module_name in modules.iter() {
            lib_rs.write_all("pub mod ".as_bytes()).await?;
            lib_rs.write_all(module_name.as_bytes()).await?;
            lib_rs.write_all(";\n".as_bytes()).await?;
        }
        lib_rs.flush().await.map_err(|err| {
            error!(?err, "Could not flush lib.rs file after writing.");
            err
        })?;

        let features = {
            let mut lock = features.write().await;
            let num_features = lock.len();
            info!(num_features, "Sorting features to avoid churn.");
            lock.sort();
            std::mem::take(&mut *lock)
        };
        self.cargo_toml.append_features(features).await?;

        info!("Writing README.md.");
        self.readme_md.write_usage().await?;
        self.readme_md.write_package_table().await?;
        self.readme_md.write_contribution().await?;

        info!("Writing ICONS.md.");
        let package_icon_metadata = {
            let mut lock = package_icon_metadata.write().await;
            std::mem::take(&mut *lock)
        };
        self.icons_md
            .write_icon_table(package_icon_metadata)
            .await?;

        Ok(())
    }
}
