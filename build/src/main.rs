use anyhow::Result;
use clap::{command, Parser};
use tracing::{error, info};
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::format::{Format, Pretty};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer, Registry};

use crate::icon_library::IconLibrary;
use crate::main_library::MainLibrary;
use crate::package::Package;

mod feature;
mod git;
mod icon;
mod icon_library;
mod main_library;
mod package;
mod path;
mod sem_ver;

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

    let handles = Package::all()
        .into_iter()
        .map(|package| {
            tokio::spawn(async move {
                if args.clean {
                    package.remove().await?;
                }

                // Download the package.
                let package_type = package.ty;
                let package = package.download().map_err(|err| {
                    error!(
                        ?package_type,
                        ?err,
                        "Downloading the package failed unexpectedly."
                    );
                    err
                })?;

                // Generate the library for that package.
                let lib_name = format!("leptos-icons-{}", package.meta.short_name);
                let lib_path = path::library_crate(&lib_name, "");
                let mut lib = IconLibrary::new(package, lib_name, lib_path);

                lib.generate().await?;

                Ok::<IconLibrary, anyhow::Error>(lib)
            })
        })
        .collect::<Vec<_>>();

    let libs = {
        let mut libs = Vec::new();
        for handle in handles {
            match handle.await.unwrap() {
                Ok(lib) => libs.push(lib),
                Err(err) => {
                    error!(?err, "Could not process package successfully.");
                    return Err(err);
                }
            }
        }
        libs
    };

    let num_libs = libs.len();

    let lib_name = "leptos-icons".to_owned();
    let lib_path = path::library_crate(&lib_name, "");
    let mut main_lib = MainLibrary::new(lib_name, lib_path);
    main_lib.generate(libs).await?;

    let end = time::OffsetDateTime::now_utc();
    info!(
        took = format!("{}s", (end - start).whole_seconds()),
        num_libs, "Build successful!"
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

/// Simply tests that from the assumed repository root, both the "build" and "leptos-icons" directories are visible.
/// This may prevent unwanted file operations in wrong directories.
fn assert_paths() {
    let build_crate_root = path::build_crate("");
    let leptos_icons_crate_root = path::library_crate("leptos-icons", "");
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
