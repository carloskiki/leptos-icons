use anyhow::Result;
use clap::{command, Parser};
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

use crate::feature::Feature;
use crate::library::Library;

mod download;
mod feature;
mod icon;
mod leptos;
mod library;
mod optimize;
mod package;
mod parse;
mod path;
mod sem_ver;

// Missing support for:
// - Docs
// - props passing
// - optimizing svgs
// - ssr optimizations?

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct BuildArgs {
    /// Clear downloads and re-download.
    #[arg(long, default_value_t = false)]
    clean: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing(tracing::level_filters::LevelFilter::INFO);

    assert_paths();

    let args: BuildArgs = BuildArgs::parse();
    info!(?args, "Parsed program arguments.");

    let start = time::OffsetDateTime::now_utc();

    let lib = Library::new();

    info!("Resetting library directory.");
    lib.src_dir().reset().await?;
    lib.cargo_toml().remove().await?;
    lib.cargo_toml().init().await?;

    let features = Arc::new(RwLock::new(Vec::<Feature>::new()));
    let modules = Arc::new(RwLock::new(Vec::<String>::new()));

    let handles = package::Package::iter()
        .map(|package| (package, package.metadata()))
        .map(|(package, meta)| {
            let features = features.clone();
            let modules = modules.clone();
            tokio::spawn(async move {
                // Remove previous download if requested.
                if args.clean {
                    download::remove_package(&meta).await?;
                }

                // Download the package.
                download::download_package(&meta).map_err(|err| {
                    error!(
                        ?package,
                        ?err,
                        "Downloading the package failed unexpectedly."
                    );
                    err
                })?;

                // Extract icon information from that package.
                // Sorting the resulting Vec is necessary, as we want to reduce churn in the later generated output as much as possible.
                let mut icons = parse::get_icons(package, &meta).await.map_err(|err| {
                    error!(?package, ?err, "Could not get icons.");
                    err
                })?;
                icons.sort_by(|icon_a, icon_b| icon_a.component_name.cmp(&icon_b.component_name));

                // Collect feature names for this icon-package.
                info!(?package, "Collecting feature names.");
                {
                    let mut lock = features.write().await;
                    for icon in &icons {
                        lock.push(icon.feature.clone());
                    }
                }

                info!(?package, "Collecting module name.");
                {
                    let mut lock = modules.write().await;
                    lock.push(meta.short_name.clone().into_owned());
                }

                // Generate leptos icon components. Note that these sorted correctly, as the icons were already sorted.
                info!(?package, "Generating leptos icon components.");
                let icon_components = icons
                    .into_iter()
                    .map(|icon| {
                        icon.create_leptos_icon_component().unwrap() // TODO:: Error handling
                    })
                    .collect::<Vec<_>>();

                // Writing leptos icon components.
                info!(
                    ?package,
                    num_components = icon_components.len(),
                    "Writing leptos icon components."
                );
                let mut mod_path = path::leptos_icons_crate("src").join(meta.short_name.as_ref()); // TODO: This should also be done using the lib type. Potential for tracking created modules.
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
                // TODO: Once https://github.com/leptos-rs/leptos/pull/748 is merged, this write can be removed. In component generation use `::leptos::...` wherever possible.
                mod_file_writer
                    .write_all("use leptos::*;\n\n".as_bytes())
                    .await
                    .unwrap();
                for comp in icon_components {
                    mod_file_writer.write_all(comp.0.as_bytes()).await.unwrap();
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

    let mut modules = modules.write().await;
    let num_modules = modules.len();

    info!(num_modules, "Sorting modules to avoid churn.");
    modules.sort();

    info!(num_modules, "Writing modules to lib.rs.");
    let mut lib_rs = lib.src_dir().lib_rs().append().await?;
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
    lib.cargo_toml().append_features(features).await?;

    let end = time::OffsetDateTime::now_utc();
    info!(
        took = format!("{}s", (end - start).whole_seconds()),
        "Build successful!"
    );

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
