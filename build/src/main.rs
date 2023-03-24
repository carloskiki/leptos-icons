use anyhow::Result;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;
use strum::IntoEnumIterator;
use tracing::{error, info};
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::format::{Format, Pretty};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer, Registry};

use generate::gen_lib_files;
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

fn main() -> Result<()> {
    init_tracing(tracing::level_filters::LevelFilter::INFO);

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

    clean_src_dir()?;
    generate::generate_initial_cargo_toml()?;

    let packages = package::Package::iter()
        .map(|package| (package, package.metadata()))
        .collect::<Vec<_>>();

    // Re-download packages
    download::clear()?;
    packages
        .par_iter()
        .map(|(package, meta)| {
            download::download_package(&meta).map_err(|err| {
                error!(?package, ?err, "Downloading the package failed unexpectedly.");
                err
            })
        })
        .collect::<Result<()>>()?;

    info!("All packages successfully downloaded!");

    packages
        .par_iter()
        .map(|(package, meta)| {
            let icons = parse::get_icons(*package, meta, Path::new("")).map_err(|err| {
                error!(?package, ?err, "Could not get icons.");
                err
            })?;

            // Generate Lib files
            let mut lib_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(src_path("lib.rs"))
                .map_err(|err| {
                    error!(?package, ?err, "Could not generate lib file.");
                    err
                })?;

            info!("All packages successfully downloaded!");

            let mut cargo_file = BufWriter::new(
                OpenOptions::new()
                    .append(true)
                    .open(path::leptos_icons_crate("Cargo.toml"))
                    .map_err(|err| {
                        error!(
                            ?package,
                            ?err,
                            "Could not open Cargo.toml file to append data."
                        );
                        err
                    })?,
            );

            gen_lib_files(*package, meta, icons, &mut lib_file, &mut cargo_file).map_err(
                |err| {
                    error!(?err, "Could not generate lib files.");
                    err
                },
            )?;

            cargo_file.flush().map_err(|err| {
                error!(
                    ?package,
                    ?err,
                    "Could not flush Cargo.toml file after writing."
                );
                err
            })?;

            Ok(())
        })
        .progress()
        .collect::<Result<()>>()?;

    info!("Build successful!");

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
