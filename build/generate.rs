use anyhow::{anyhow, Result};
use std::{
    fs::{create_dir, create_dir_all, File, OpenOptions},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use crate::{
    path::src_path,
    types::{Icon, IconPackage},
};

pub(crate) fn gen_lib_files(
    icon_package: &IconPackage,
    lib_file: &mut File,
    cargo_file: &mut BufWriter<File>,
) -> Result<()> {
    let mut package_path = src_path(&icon_package.short_name);
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
                &icon,
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

fn declare_icon(file: &mut File, mod_name: &str, feature_name: &str) -> Result<()> {
    write!(
        file,
        "#[cfg(feature = {})]\npub mod {};\n",
        feature_name, mod_name
    )
    .map_err(anyhow::Error::from)
}

fn create_modules_on_path(module_path: &PathBuf, package_file: &mut File) -> Result<()> {
    let mut full_module_path = src_path(module_path);
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
        let mut module_file_path = src_path(ancestor);
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

    let mut icon_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&icon_path.join(format!("{}.rs", &icon_component_name)))?;

    write!(
        &mut icon_file,
        "#[cfg(feature = \"{icon_feature_name}\")]
use leptos::{{component, Scope, IntoView, view}};

#[cfg(feature = \"{icon_feature_name}\")]
/// *This icon requires the feature* `{icon_feature_name}` *to be enabled*.
#[component]
pub fn {}(cx: Scope) -> impl IntoView {{
   view! {{ cx,
       {}
   }}
}}",
        &icon_component_name, &icon.content
    )?;

    icon_path.set_extension("rs");
    let mut upper_module_file = OpenOptions::new().append(true).open(&icon_path)?;
    declare_icon(&mut upper_module_file, &icon_component_name, &icon_feature_name)?;

    cargo_file.write_all(format!("{} = []\n", &icon_feature_name).as_bytes())?;

    Ok(())
}
