use anyhow::{Result, anyhow};
use std::{fs::{read_dir, read_to_string, File}, path::Path, str::FromStr};

use crate::{types::{IconPackage, Icon, IconSize, IconName}, optimize::optimize};

pub(crate) fn get_icons(icon_package: &mut IconPackage, extra_path: &Path) -> Result<()> {
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
                let name = format_icon_name(
                        entry_relative_path
                            .to_str()
                            .ok_or(anyhow!("icon path is not a valid utf-8 path"))?
                            .to_string(),
                        &icon_package.short_name,
                    )?;
                let content = optimize(File::open(entry_full_path)?)?;
                icon_package.icons.push(Icon {
                    content,
                    name,
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
