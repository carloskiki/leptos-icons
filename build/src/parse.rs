use anyhow::{anyhow, Result};
use std::{fs::File, path::Path, str::FromStr};
use tracing::{error, info, instrument};

use crate::{
    icon::{Category, Icon, IconName, IconSize},
    optimize,
    package::{Package, PackageMetadata},
    path,
};

#[instrument(level = "info", skip(meta))]
pub(crate) fn get_icons(
    package: Package,
    meta: &PackageMetadata,
    extra_path: &Path,
) -> Result<Vec<Icon>> {
    info!("Retrieving icon names...");
    let svg_dir = path::download_path(meta.download_dir.as_ref())
        .join(meta.svg_dir.as_ref())
        .join(extra_path);

    let mut icons = Vec::new();

    for entry in std::fs::read_dir(&svg_dir)? {
        let entry = entry?;
        let entry_full_path = entry.path();
        let entry_name: &Path = &entry_full_path.strip_prefix(&svg_dir)?;
        let entry_relative_path = extra_path.join(entry_name);

        if !entry_full_path.exists() {
            error!(
                ?entry_full_path,
                "There is something wrong here... Path does not exist"
            );
            panic!("Path does not exist.");
        }

        if entry.file_type()?.is_dir() {
            get_icons(package, meta, &entry_relative_path)?;
        };

        if let Some(file_extension) = entry_name.extension() {
            let file_extension = file_extension.to_str().ok_or(anyhow!(
                "not a valid file extension (could not convert OsStr)"
            ))?;
            if file_extension == "svg" {
                let name = format_icon_name(
                    package,
                    entry_relative_path
                        .to_str()
                        .ok_or(anyhow!("icon path is not a valid utf-8 path"))?
                        .to_string(),
                )?;
                let content = optimize::optimize(File::open(entry_full_path)?)?;
                icons.push(Icon { content, name });
            }
        }
    }

    info!(num_icons = icons.len(), "Finished retrieving icon names.");
    Ok(icons)
}

fn format_icon_name(package: Package, icon_path_string: String) -> Result<IconName> {
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
                categories.push(Category::Other(dir_name.to_lowercase()));
                (size, categories)
            });

    Ok(IconName::new(package, file_name, icon_size, categories))
}
