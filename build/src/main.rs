use anyhow::Result;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use strum::IntoEnumIterator;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;
use tracing::{error, info};
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::format::{Format, Pretty};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer, Registry};

use path::src_path;

mod download;
mod generate;
mod icon;
mod optimize;
mod package;
mod parse;
mod path;
mod sem_ver;

// Not working
// - two file with same name
// - files starting with digits

// Missing support for:
// - Docs
// - props passing
// - optimizing svgs
// - remove useless categories (e.g. vscode-light/dark, sizes?)
// - ssr optimizations?

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing(tracing::level_filters::LevelFilter::INFO);

    assert_paths();

    let start = time::OffsetDateTime::now_utc();

    // Re-download packages
    download::clear().await?;
    clean_src_dir()?;
    generate::generate_initial_cargo_toml()?;

    //let lib_mods = Arc::new(RwLock::new(Vec::<String>::new()));
    let features = Arc::new(RwLock::new(Vec::<String>::new()));
    let modules = Arc::new(RwLock::new(Vec::<String>::new()));

    let handles = package::Package::iter()
        .map(|package| (package, package.metadata()))
        .into_iter()
        .map(|(package, meta)| {
            let features = features.clone();
            let modules = modules.clone();
            tokio::spawn(async move {
                // 1. Download the package.
                download::download_package(&meta).map_err(|err| {
                    error!(
                        ?package,
                        ?err,
                        "Downloading the package failed unexpectedly."
                    );
                    err
                })?;

                // 2. Extract icon information from that package.
                let mut icons = parse::get_icons(package, &meta).await.map_err(|err| {
                    error!(?package, ?err, "Could not get icons.");
                    err
                })?;
                // Sorting is necessary, as we wan't to reduce churn as much as possible.
                icons.sort_by(|icon_a, icon_b| icon_a.component_name.cmp(&icon_b.component_name));


                // 3. Collect feature names for this icon-package. TODO: This should be port / outcome of another process..
                info!(?package, "Generating feature names.");
                let mut lock = features.write().await;
                for icon in &icons {
                    lock.push(icon.feature_name.clone()); // TODO: remove clone
                }
                drop(lock);

                info!(?package, "Keeping module name.");
                let mut lock = modules.write().await;
                lock.push(meta.short_name.to_owned().to_string());
                drop(lock);


                // 4. Generate and write leptos-icon components.
                info!(?package, "Generating leptos icon component declarations.");
                // NOTE: sorted correctly, as icons were already sorted.
                let icon_components = generate::gen_icon_components(package, icons);

                // 4. Generate and write leptos-icon components.
                let mut mod_path = src_path(meta.short_name.as_ref());
                mod_path.set_extension("rs");
                let mut mod_file_writer = tokio::io::BufWriter::new(
                    tokio::fs::OpenOptions::new()
                        .create(true)
                        .write(true)
                        .open(mod_path)
                        .await
                        .map_err(|err| {
                            error!(?package, ?err, "Could not open mod file.");
                            err
                        })?,
                );
                for comp in icon_components {
                    mod_file_writer.write(comp.as_bytes()).await.unwrap();
                }

                Ok::<(), anyhow::Error>(())
            })
        })
        .collect::<Vec<_>>();
    for handle in handles {
        handle.await.unwrap().unwrap();
    }

    let mut modules = modules.write().await;
    let num_modules = modules.len();

    info!(num_modules, "Sorting modules to avoid churn.");
    modules.sort();

    info!(num_modules, "Writing modules to lib.rs.");
    let mut lib_file = tokio::io::BufWriter::new(
        tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path::src_path("lib.rs"))
            .await
            .map_err(|err| {
                error!(?err, "Could not open lib.rs file to append modules.");
                err
            })?,
    );
    for module_name in modules.iter() {
        lib_file.write_all("pub mod ".as_bytes()).await?;
        lib_file.write_all(module_name.as_bytes()).await?;
        lib_file.write_all(";\n".as_bytes()).await?;
    }
    lib_file.flush().await.map_err(|err| {
        error!(?err, "Could not flush lib.rs file after writing.");
        err
    })?;

    let mut features = features.write().await;
    let num_features = features.len();

    info!(num_features, "Sorting features to avoid churn.");
    features.sort();

    info!(num_features, "Writing features to Cargo.toml.");
    let mut cargo_file = tokio::io::BufWriter::new(
        tokio::fs::OpenOptions::new()
            .append(true)
            .open(path::leptos_icons_crate("Cargo.toml"))
            .await
            .map_err(|err| {
                error!(?err, "Could not open Cargo.toml file to append data.");
                err
            })?,
    );
    for feature_name in features.iter() {
        cargo_file
            .write_all(format!("{} = []\n", &feature_name).as_bytes())
            .await?;
    }
    cargo_file.flush().await.map_err(|err| {
        error!(?err, "Could not flush Cargo.toml file after writing.");
        err
    })?;

    let end = time::OffsetDateTime::now_utc();
    let took = end - start;
    let took = format!("{}s", took.whole_seconds());
    info!(took, "Build successful!");

    Ok(())
}

fn init_tracing(level: tracing::level_filters::LevelFilter) {
    fn build_log_filter(default_log_level: tracing::level_filters::LevelFilter) -> Targets {
        Targets::new().with_default(default_log_level)
    }

    fn build_tracing_subscriber_fmt_layer(
    ) -> tracing_subscriber::fmt::Layer<Registry, Pretty, Format<Pretty>> {
        tracing_subscriber::fmt::layer()
            .pretty()
            .with_file(true)
            .with_line_number(true)
            .with_ansi(true)
            .with_thread_names(false)
            .with_thread_ids(false)
    }

    let fmt_layer_filtered =
        build_tracing_subscriber_fmt_layer().with_filter(build_log_filter(level));

    Registry::default().with(fmt_layer_filtered).init();
}

fn assert_paths() {
    let build_crate_root = path::build_crate("");
    let leptos_icons_crate_root = path::leptos_icons_crate("");
    info!(?build_crate_root, "Using");
    info!(?leptos_icons_crate_root, "Using");

    assert_eq!(
        Some("build"),
        build_crate_root.file_name().and_then(|it| it.to_str())
    );
    assert_eq!(
        Some("leptos-icons"),
        leptos_icons_crate_root
            .file_name()
            .and_then(|it| it.to_str())
    );
}

fn clean_src_dir() -> Result<()> {
    let src_dir = path::src_path("");

    info!(?src_dir, "Removing existing src dir");
    std::fs::remove_dir_all(&src_dir)?;

    info!(?src_dir, "Creating new src dir");
    std::fs::create_dir(&src_dir)?;

    let lib_rs_path = path::src_path("lib.rs");

    info!(?lib_rs_path, "Creating new lib.rs file");
    OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(lib_rs_path)?
        .write("#![allow(non_snake_case)]\n".as_bytes())?;

    Ok(())
}
