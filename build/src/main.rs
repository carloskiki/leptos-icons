use anyhow::Result;
use clap::{command, Parser};
use tracing::info;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::format::{Format, Pretty};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer, Registry};

use crate::library::Library;

mod feature;
mod icon;
mod git;
mod library;
mod package;
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

    let lib = Library::new(path::leptos_icons_crate(""));
    lib.generate(args.clean).await?;

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

/// Simply tests that from the assumed repository root, both the "build" and "leptos-icons" directories are visible.
/// This may prevent unwanted file operations in wrong directories.
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
