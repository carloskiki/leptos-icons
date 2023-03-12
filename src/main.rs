use anyhow::{Result, anyhow};
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

use path::{src_path, crate_path};
use types::IconPackage;
use submodules::download_submodule;
use generate::gen_lib_files;
use parse::get_icons;

mod path;
mod submodules;
mod types;
mod parse;
mod generate;
mod optimize;

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
    let raw_icon_packages =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/icon-packages.json"));

    let mut cargo_file = OpenOptions::new()
        .append(true)
        .open(crate_path("Cargo.toml"))?;

    write!(
        cargo_file,
        "
[features]
"
    )?;

    let mut icon_packages: Vec<IconPackage> = serde_json::from_str(raw_icon_packages).unwrap();
    icon_packages
        .par_iter_mut()
        .map(|icon_package| {
            // Download icon packages
            // download_submodule(&icon_package).and_then(|exit_status| {
            //     exit_status
            //         .success()
            //         .then_some(())
            //         .ok_or(anyhow!("submodule was not downloded successfully"))
            // })?;

            // Get Icons Paths
            get_icons(icon_package, Path::new(""))?;

            // Generate Lib files
            let mut lib_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(src_path("lib.rs"))?;
            let mut cargo_file = BufWriter::new(
                OpenOptions::new()
                    .append(true)
                    .open(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))?,
            );

            gen_lib_files(icon_package, &mut lib_file, &mut cargo_file)?;

            cargo_file.flush()?;

            Ok(())
        })
        .progress()
        .collect::<Result<()>>()
}
