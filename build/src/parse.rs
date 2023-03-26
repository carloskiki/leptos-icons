use anyhow::Result;
use std::{path::PathBuf, str::FromStr};
use tracing::{info, instrument, warn};

use crate::{
    icon::{self, Icon, IconSize},
    optimize,
    package::{Package, PackageMetadata},
    path,
};

#[instrument(level = "info", skip(meta))]
pub(crate) async fn get_icons(package: Package, meta: &PackageMetadata) -> Result<Vec<Icon>> {
    info!("Retrieving icon names...");
    let mut icons = Vec::new();

    // A vec of directories to search combined with a list of categories valid for the contents of that directory.
    let mut search_dirs = Vec::<(PathBuf, Vec<String>)>::new();
    search_dirs.push((
        path::download_path(meta.download_dir.as_ref()).join(meta.svg_dir.as_ref()),
        Vec::new(),
    ));

    while !search_dirs.is_empty() {
        let (current, categories) = search_dirs.pop().expect("must exist");
        let mut dir_stream = tokio::fs::read_dir(&current).await?;
        while let Some(entry) = dir_stream.next_entry().await? {
            let entry_path = entry.path();

            if entry.file_type().await?.is_dir() {
                info!(additional_dir = ?entry_path, "Found additional directory.");

                // This dir must be searched as well. We consider is's name as being a "category" for the items contained in it.
                let cat = entry_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                let mut entry_cats = categories.clone();
                entry_cats.push(cat);
                search_dirs.push((current.join(&entry_path), entry_cats));

                continue;
            };

            match entry_path.extension() {
                Some(file_extension) => {
                    match file_extension.to_str() {
                        Some(file_extension) => match file_extension {
                            "svg" => {
                                let file_stem = entry_path.file_stem().unwrap().to_string_lossy(); // TODO: Error handling\

                                let (raw_name, size_from_name, cats_from_name) =
                                    icon::extract_raw_icon_name(package, &file_stem);

                                let mut icon_categories = categories.clone();
                                if let Some(mut cats_from_name) = cats_from_name {
                                    icon_categories.append(&mut cats_from_name);
                                }

                                let icon_size = size_from_name.or_else(|| {
                                    icon_categories
                                        .iter()
                                        .filter_map(|cat| IconSize::from_str(&cat).ok())
                                        .next()
                                });

                                let feature_name = icon::feature_name(
                                    raw_name,
                                    icon_size,
                                    &icon_categories,
                                    &meta.short_name,
                                );

                                let view = optimize::optimize(
                                    tokio::fs::read_to_string(&entry_path).await?.as_bytes(),
                                )?;

                                icons.push(Icon {
                                    view,
                                    size: icon_size,
                                    categories: icon_categories,
                                    component_name: feature_name.clone(), // TODO: Make clear why
                                    feature_name,
                                });
                            }
                            _ => warn!(
                                ?entry_path,
                                file_extension, "Found file without svg extension. Ignoring it."
                            ),
                        },
                        None => {
                            warn!(
                                ?entry_path,
                                ?file_extension,
                                "Found file whose file_extension (&OsStr) could not be converted to a &str. Ignoring it."
                            );
                        }
                    }
                }
                None => warn!(?entry_path, "Found file without extension. Ignoring it."),
            };
        }
    }

    info!(num_icons = icons.len(), "Finished retrieving icon names.");
    Ok(icons)
}
