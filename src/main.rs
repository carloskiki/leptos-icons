use anyhow::{anyhow, Result};
use rayon::prelude::*;
use serde::Deserialize;
use std::fs::{create_dir, read_dir, read_to_string, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use url::Url;

// Missing support for:
// 1. props passing
// 2. optimizing svg
// 3. remove useless categories (e.g. vscode-light/dark, sizes?)
// 4. ssr optimizations?

fn main() -> Result<()> {
    // 1. Download Submodules
    // 2. Get Icons
    // 3. Write features
    // 4. Write setup features

    let raw_icon_packages = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/icons/icon-packages.json"
    ));

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
                .open(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))?;
            let mut cargo_file = OpenOptions::new()
                .append(true)
                .open(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))?;

            gen_lib_files(&icon_package, &mut lib_file, &mut cargo_file)?;

            Ok(())
        })
        .collect::<Result<()>>()?;

    Ok(())
}

fn gen_lib_files(icon_package: &IconPackage, lib_file: &mut File, cargo_file: &mut File) -> Result<()> {
    // create package dir
    let mut package_path = format!(
        "{}/src/{}",
        env!("CARGO_MANIFEST_DIR"),
        icon_package.short_name,
    );
    create_dir(&package_path)?;

    // create package file
    package_path.push_str(".rs");
    let package_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(package_path)?;

    // declare package in lib
    write!(lib_file, "pub mod {};\n", icon_package.short_name)?;

    Ok(())
}

fn get_icons(icon_package: &mut IconPackage, extra_path: &Path) -> Result<()> {
    let path_to_package = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(&icon_package.path)
        .join(extra_path);

    for entry in read_dir(&path_to_package)? {
        let entry = entry?;
        let entry_full_path = entry.path();
        let entry_name: &Path = &entry_full_path.strip_prefix(&path_to_package)?;
        let entry_relative_path = extra_path.join(entry_name);

        if entry.file_type()?.is_dir() {
            get_icons(icon_package, &entry_relative_path)?;
        };

        if let Some(file_extension) = entry_name.extension() {
            let file_extension = file_extension.to_str().ok_or(anyhow!(
                "not a valid file extension (could not convert OsStr)"
            ))?;
            if file_extension == "svg" {
                icon_package.icons.push(Icon {
                    content: read_to_string(&entry_full_path)?,
                    name: format_icon_name(
                        entry_relative_path
                            .to_str()
                            .ok_or(anyhow!("icon path is not a valid utf-8 path"))?
                            .to_string(),
                        &icon_package.short_name,
                    )?,
                });
            }
        }
    }

    Ok(())
}

fn format_icon_name(icon_path_string: String, package_short_name: &str) -> Result<IconName> {
    let mut directories = icon_path_string[..(icon_path_string.len() - 4)]
        .split('/')
        .collect::<Vec<&str>>();
    let file_name = directories.pop().ok_or(anyhow!("icon path is empty"))?;

    let (icon_size, categories) =
        directories
            .iter()
            .fold((None, Vec::new()), |(size, mut categories), dir_name| {
                if dir_name.chars().all(char::is_numeric) {
                    return (IconSize::from_str(&dir_name).ok(), categories);
                };
                categories.push(dir_name.to_lowercase());
                (size, categories)
            });

    let icon_name = IconName::new(file_name, package_short_name, icon_size, categories);

    Ok(icon_name)
}

fn download_submodule(icon_package: &IconPackage) -> Result<ExitStatus> {
    Command::new("git")
        .args([
            "submodule",
            "add",
            "-f",
            "--depth",
            "1",
            icon_package.url.as_str(),
            &format!("./icons/{}", icon_package.folder_name),
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .map_err(anyhow::Error::from)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct IconPackage {
    short_name: String,
    package_name: String,
    license: String,
    url: Url,
    path: PathBuf,
    folder_name: String,
    #[serde(skip)]
    icons: Vec<Icon>,
}

#[derive(Debug)]
struct Icon {
    content: String,
    name: IconName,
}

#[derive(Debug)]
enum IconSize {
    Sm,
    Md,
    Lg,
}

impl IconSize {
    fn from_str(size_string: &str) -> Result<IconSize> {
        match size_string {
            "16" => Ok(IconSize::Sm),
            "20" => Ok(IconSize::Md),
            "24" => Ok(IconSize::Lg),
            _ => return Err(anyhow!("icon size not recognized")),
        }
    }
}

#[derive(Debug)]
struct IconName {
    size: Option<IconSize>,
    categories: Vec<String>,
    name: String,
}

// Ant design icons: filled, outlined, twotone
// Font awesome: brands, regular, solid
// Feather icons: Not categorized
// VS code icons: light, dark
// Bootstrap icons: Not categorized
// Ionicons: Not categorized
// Remix icons: tons of categories
// Simple icons: Not categorized
// Typicons: Not categorized
// Heroicons: (20| solid) , (24| outline, solid)
// css.gg: Not categorized
// Tabler icons: Not categorized
impl IconName {
    fn new(
        mut name: &str,
        package_short_name: &str,
        mut size: Option<IconSize>,
        mut categories: Vec<String>,
    ) -> IconName {
        match package_short_name {
            // octoicons: size suffix e.g: '-24.svg'
            "oc" => {
                size = IconSize::from_str(&name[(name.len() - 2)..]).ok();
                name = name
                    .trim_end_matches(char::is_numeric)
                    .trim_end_matches('-');
            }
            // Weather icons: 'wi-' prefix
            "wi" => {
                name = name.trim_start_matches("wi-");
            }
            // Box icons: logos: 'bxl-', regular:  'bx-', solid: 'bxs-' prefixes
            "bi" => {
                name = name
                    .trim_start_matches("bxl-")
                    .trim_start_matches("bx-")
                    .trim_start_matches("bxs-");
            }
            // Icomoon icons: numbered '001-xxxxxx'
            "im" => {
                name = name.trim_start_matches(char::is_numeric);
            }
            "ri" => {
                if name.ends_with("-fill") {
                    name = name.trim_end_matches("-fill");
                    categories.push("fill".to_string());
                } else if name.ends_with("-line") {
                    name = name.trim_end_matches("-line");
                    categories.push("line".to_string());
                }
            }
            _ => (),
        };

        let name = name.to_string();

        IconName {
            size,
            categories,
            name,
        }
    }

    // fn set_size(&mut self, size: &str) -> Result<()> {
    //     self.size = Some(IconSize::from_str(size)?);
    //     Ok(())
    // }

    // fn set_category(&mut self, category: &str) -> Result<()> {
    //     let mut chars = category.chars();
    //     let category = match chars.next() {
    //         None => return Err(anyhow!("the category string is empty")),
    //         Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    //     };

    //     self.category = Some(category);

    //     Ok(())
    // }
}
