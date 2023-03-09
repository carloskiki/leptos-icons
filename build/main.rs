use anyhow::{anyhow, Context, Result};
use convert_case::{Case, Casing};
use rayon::prelude::*;
use serde::Deserialize;
use std::fs::{create_dir, create_dir_all, read_dir, read_to_string, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::str::FromStr;
use url::Url;

// Not working
// 1. two file with same name
// 2. files starting with digits

// Missing support for:
// 1. props passing
// 2. optimizing svgs
// 3. remove useless categories (e.g. vscode-light/dark, sizes?)
// 4. ssr optimizations?
// 5. Docs

fn main() -> Result<()> {
    let raw_icon_packages =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/icon-packages.json"));

    let mut cargo_file = OpenOptions::new()
        .append(true)
        .open(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))?;

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
                .open(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))?;
            let mut cargo_file = BufWriter::new(
                OpenOptions::new()
                    .append(true)
                    .open(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))?,
            );

            gen_lib_files(icon_package, &mut lib_file, &mut cargo_file)?;

            cargo_file.flush()?;

            Ok(())
        })
        .collect::<Result<()>>()
}

fn pkg_path_to_abs<P: AsRef<Path>>(relative_path: P) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/")
        .join(relative_path)
}

fn gen_lib_files(
    icon_package: &IconPackage,
    lib_file: &mut File,
    cargo_file: &mut BufWriter<File>,
) -> Result<()> {
    let mut package_path = pkg_path_to_abs(&icon_package.short_name);
    create_dir(&package_path)?;

    package_path.set_extension("rs");
    let mut package_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(package_path)?;

    declare_mod(lib_file, &icon_package.short_name)?;

    let mut modules_created: Vec<PathBuf> = vec![PathBuf::from(&icon_package.short_name)];
    icon_package
        .icons
        .iter()
        .map(|icon| {
            let mut path_to_icon = PathBuf::from(&icon_package.short_name);
            if let Some(size) = icon.name.size {
                let size_string = size.to_string();
                path_to_icon.push(&size_string);
            }

            icon.name.categories.iter().for_each(|category| {
                path_to_icon.push(category);
            });

            if !modules_created.contains(&path_to_icon) {
                create_modules_on_path(&path_to_icon, &mut package_file)?;
                modules_created.push(path_to_icon.clone());
            };

            create_icon(
                Path::new("src/").join(path_to_icon),
                icon,
                &icon_package.short_name,
                cargo_file,
            )?;

            Ok(())
        })
        .collect::<Result<()>>()
}

fn declare_mod(file: &mut File, mod_name: &str) -> Result<()> {
    write!(file, "pub mod {};\n", mod_name).map_err(anyhow::Error::from)
}

fn create_modules_on_path(module_path: &PathBuf, package_file: &mut File) -> Result<()> {
    let mut full_module_path = pkg_path_to_abs(module_path);
    create_dir_all(&full_module_path)?;

    full_module_path.set_extension("rs");
    OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(full_module_path)?;

    let mut new_child_module = module_path
        .file_stem()
        .unwrap()
        .to_str()
        .ok_or(anyhow!("file name is not valid utf-8"))?
        .to_owned();

    for ancestor in module_path.ancestors().skip(1) {
        let mut module_file_path = pkg_path_to_abs(ancestor);
        module_file_path.set_extension("rs");
        let file_existed = module_file_path.exists();

        let mut module_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&module_file_path)?;

        declare_mod(&mut module_file, &new_child_module)?;

        if file_existed {
            return Ok(());
        };

        new_child_module = module_file_path
            .file_stem()
            .unwrap()
            .to_str()
            .ok_or(anyhow!("file name is not valid utf-8"))?
            .to_owned();
    }

    let first_category = module_path.iter().next().unwrap().to_str().unwrap();
    declare_mod(package_file, first_category)?;

    Ok(())
}

fn create_icon(
    mut icon_path: PathBuf,
    icon: &Icon,
    package_short_name: &str,
    cargo_file: &mut BufWriter<File>,
) -> Result<()> {
    let icon_component_name = icon.name.component_name();
    let icon_feature_name = icon.name.feature_name(package_short_name);

    if icon_feature_name == "" {
        println!("feature name for {} is empty", &icon_component_name);
    };

    let mut icon_file = OpenOptions::new()
        .create(true)
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
        &icon_feature_name, &icon_component_name, &icon.content
    )?;

    icon_path.set_extension("rs");
    let mut upper_module_file = OpenOptions::new().append(true).open(&icon_path)?;
    declare_mod(&mut upper_module_file, &icon_component_name)?;

    cargo_file.write_all(format!("{} = []\n", &icon_feature_name).as_bytes())?;

    Ok(())
}

fn get_icons(icon_package: &mut IconPackage, extra_path: &Path) -> Result<()> {
    let path_to_package = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(&icon_package.path)
        .join(extra_path);

    for entry in
        read_dir(&path_to_package).context(format!("package: {}", icon_package.package_name))?
    {
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
    Xl,
    Xxl
}

impl ToString for IconSize {
    fn to_string(&self) -> String {
        match self {
            IconSize::Sm => "sm".to_string(),
            IconSize::Md => "md".to_string(),
            IconSize::Lg => "lg".to_string(),
            IconSize::Xl => "xl".to_string(),
            IconSize::Xxl => "xxl".to_string(),
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
            "48" => Ok(IconSize::Xl),
            "96" => Ok(IconSize::Xxl),
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

    fn feature_name(&self, package_short_name: &str) -> String {
        let mut name = package_short_name.to_owned() + " ";

        if let Some(size) = self.size {
            name = name + &size.to_string() + " ";
        };
        self.categories.iter().for_each(|category| {
            name.push_str(category);
            name.push(' ');
        });
        name.push_str(&self.name);

        name.to_case(Case::Pascal)
    }
}
