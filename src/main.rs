use anyhow::{anyhow, Result, Context};
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::Command;

use generate::gen_lib_files;
use parse::get_icons;
use path::{crate_path, src_path};
use submodules::download_submodule;
use types::IconPackage;

mod generate;
mod optimize;
mod parse;
mod path;
mod submodules;
mod types;

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

    println!("cargo:rerun-if-changed=build/");

    let raw_icon_packages =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/icon-packages.json"));
    let mut icon_packages: Vec<IconPackage> = serde_json::from_str(raw_icon_packages).unwrap();

    // Reset the repository
    clean_lib()?;

    // Git commands can't be run in parralel
    icon_packages.iter().map(|icon_package| {
            // Download icon packages
            download_submodule(&icon_package).and_then(|exit_status| {
                exit_status
                    .success()
                    .then_some(())
                    .ok_or(anyhow!("submodule was not downloded successfully"))
            })
    }).collect::<Result<()>>()?;

    icon_packages
        .par_iter_mut()
        .map(|icon_package| {
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

fn clean_lib() -> Result<()> {
    // Remove git submodules
    Command::new("git").arg("rm").arg("icons/*").status()?;

    // cargo file relevant content
    let cargo_contents = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"));
    let cargo_no_features = cargo_contents.lines().take_while(|line| line != &"[features]").collect::<Vec<&str>>().join("\n");
    let cargo_path = crate_path("Cargo.toml");

    // remove old cargo file
    Command::new("rm").arg(cargo_path.to_str().unwrap()).status()?;

    // Write to new cargo file
    let mut new_cargo_file = OpenOptions::new().create_new(true).write(true).open(cargo_path)?;
    new_cargo_file.write_all(cargo_no_features.as_bytes())?;

    // remove lib files
    Command::new("rm").arg("-rf").arg(src_path("*")).status()?;

    // New lib file
    let lib_path = src_path("lib.rs");
    OpenOptions::new().create_new(true).write(true).open(lib_path)?;

    println!("test");

    Ok(())
}
