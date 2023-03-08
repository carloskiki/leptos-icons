use anyhow::{anyhow, Result, Context};
use convert_case::{Case, Casing};
use rayon::prelude::*;
use serde::Deserialize;
use std::fs::{create_dir, create_dir_all, read_dir, read_to_string, File, OpenOptions};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::str::FromStr;
use url::Url;

// Missing support for:
// 1. props passing
// 2. optimizing svg
// 3. remove useless categories (e.g. vscode-light/dark, sizes?)
// 4. ssr optimizations?

fn main() -> Result<()> {
    // 4. Write setup features

    let raw_icon_packages = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/icon-packages.json"
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

            Ok(())
        })
        .collect::<Result<()>>()?;

    let mut lib_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))?;
    let mut cargo_file = OpenOptions::new()
        .append(true)
        .open(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))?;

    println!("package: {}", icon_packages[0].package_name);

    gen_lib_files(&icon_packages[0], &mut lib_file, &mut cargo_file)?;

    Ok(())
}

fn gen_lib_files(
    icon_package: &IconPackage,
    lib_file: &mut File,
    cargo_file: &mut File,
) -> Result<()> {
    let package_path = format!(
        "{}/src/{}",
        env!("CARGO_MANIFEST_DIR"),
        icon_package.short_name,
    );
    create_dir(&package_path)?;


    let mut modules_created: Vec<PathBuf> = vec![PathBuf::new()];

    let mut package_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("{}.rs", package_path))?;

    declare_mod(lib_file, &icon_package.short_name)?;

    println!("modules created: {:?}", &modules_created);

    icon_package
        .icons
        .iter()
        .map(|icon| {
            let mut path_to_icon = PathBuf::new();
            if let Some(size) = icon.name.size {
                let size_string = size.to_string();
                path_to_icon.push(&size_string);
            }
            println!("path after size: {:?}", &path_to_icon);

            icon.name.categories.iter().for_each(|category| {
                path_to_icon.push(category);
                println!("path after category: {:?}", &path_to_icon);
            });

            if !modules_created.contains(&path_to_icon) {
                create_modules_on_path(&path_to_icon, &mut package_file)?;
                modules_created.push(path_to_icon.clone());
            };

            create_icon(PathBuf::from(&package_path).join(path_to_icon), icon)?;

            Ok(())
        })
        .collect::<Result<()>>()?;

    Ok(())
}

fn declare_mod(file: &mut File, mod_name: &str) -> Result<()> {
    write!(file, "pub mod {};\n", mod_name).map_err(anyhow::Error::from)
}

fn create_modules_on_path(module_path: &PathBuf, package_file: &mut File) -> Result<()> {
    create_dir_all(Path::new("src/").join(module_path))?;
    let mut new_child_module: Option<String> = None;
    module_path
        .ancestors()
        .map(|ancestor: &Path| {
            if let Some(child) = &new_child_module {
                let mut module_file_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/").join(ancestor);
                module_file_path.set_extension(".rs");
                println!("module file path: {:?}", &module_file_path);

                match OpenOptions::new()
                    .create_new(true)
                    .write(true)
                    .open(&module_file_path)
                {
                    Ok(mut module_file) => {
                        declare_mod(&mut module_file, &child)?;
                    }
                    Err(file_error) => {
                        if file_error.kind() != ErrorKind::AlreadyExists {
                            return Err(anyhow!("file creation error"));
                        };
                        let mut module_file =
                            OpenOptions::new().append(true).open(module_file_path)?;
                        declare_mod(&mut module_file, &child)?;
                        return Ok(());
                    }
                };
            };

            new_child_module = Some(
                ancestor
                    .file_name()
                    .unwrap()
                    .to_str()
                    .ok_or(anyhow!("module name is not a valid utf-8 string"))?
                    .to_owned(),
            );

            Ok(())
        })
        .collect::<Result<()>>()?;

        if let Some(last_child) = new_child_module {
            declare_mod(package_file, &last_child)?;
        };

        Ok(())
}

fn create_icon(mut icon_path: PathBuf, icon: &Icon) -> Result<()> {
    let icon_component_name = icon.name.component_name();
    let mut icon_file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&icon_path.join(format!("{}.rs", &icon_component_name)))?;

    write!(
        &mut icon_file,
        "use leptos::{{component, Scope, IntoView, view}};

           #[cfg(feature = {})]
           #[component]
           pub fn {}(cx: Scope) -> impl IntoView {{
               view! {{ cx,
                   {}
               }}
    }}",
        icon.name.feature_name(),
        &icon_component_name,
        &icon.content
    );

    icon_path.set_extension(".rs");
    let mut upper_module_file = OpenOptions::new().append(true).open(&icon_path)?;
    declare_mod(&mut upper_module_file, &icon_component_name)?;

    Ok(())
}

fn get_icons(icon_package: &mut IconPackage, extra_path: &Path) -> Result<()> {
    let path_to_package = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(&icon_package.path)
        .join(extra_path);

    for entry in read_dir(&path_to_package).context(format!("package: {}", icon_package.package_name))? {
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

#[derive(Clone, Copy, Debug)]
enum IconSize {
    Sm,
    Md,
    Lg,
}

impl ToString for IconSize {
    fn to_string(&self) -> String {
        match self {
            IconSize::Sm => "sm".to_string(),
            IconSize::Md => "md".to_string(),
            IconSize::Lg => "lg".to_string(),
        }
    }
}

impl FromStr for IconSize {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
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

    fn component_name(&self) -> String {
        self.name.to_case(Case::Pascal)
    }

    fn feature_name(&self) -> String {
        let mut name = String::new();
        if let Some(size) = self.size {
            name.push_str(&size.to_string().to_case(Case::Pascal));
        };
        self.categories.iter().for_each(|category| {
            name.push_str(&category.to_case(Case::Pascal));
        });
        name
    }
}
