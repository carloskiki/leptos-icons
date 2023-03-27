use anyhow::Result;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};
use tracing::{info, instrument, warn};

use crate::{
    icon::{self, Category, Icon, IconSize},
    optimize,
    package::{Package, PackageMetadata},
    path,
};

#[instrument(level = "info", skip(meta))]
pub(crate) async fn get_icons(package: Package, meta: &PackageMetadata) -> Result<Vec<Icon>> {
    info!("Retrieving icon names...");
    let mut icons = Vec::new();

    // A vec of directories to search combined with a list of categories valid for the contents of that directory.
    let mut search_dirs = Vec::<(PathBuf, Vec<Category>)>::new();
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
                let file_name = entry_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();

                let mut entry_cats = categories.clone();
                if package.is_category(&file_name) {
                    entry_cats.push(Category(file_name));
                }
                search_dirs.push((current.join(&entry_path), entry_cats));

                continue;
            };

            match entry_path.extension() {
                Some(file_extension) => match file_extension.to_str() {
                    Some(file_extension) => match file_extension {
                        "svg" => icons
                            .push(svg_icon(package, meta, &entry_path, categories.clone()).await?),
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
                },
                None => warn!(?entry_path, "Found file without extension. Ignoring it."),
            };
        }
    }

    info!(num_icons = icons.len(), "Finished retrieving icon names.");
    Ok(icons)
}

async fn svg_icon(
    package: Package,
    meta: &PackageMetadata,
    path: &Path,
    mut categories: Vec<Category>,
) -> Result<Icon> {
    let file_stem = path.file_stem().unwrap().to_string_lossy(); // TODO: Error handling\

    let (raw_name, size_from_name, cats_from_name) =
        icon::extract_raw_icon_name(package, &file_stem);

    if let Some(mut cats_from_name) = cats_from_name {
        categories.append(&mut cats_from_name);
    }

    let icon_size = size_from_name.or_else(|| {
        categories
            .iter()
            .find_map(|cat| IconSize::from_str(&cat.0).ok())
    });

    let feature_name = icon::feature_name(raw_name, icon_size, &categories, &meta.short_name);

    let view = optimize::optimize(tokio::fs::read_to_string(path).await?.as_bytes())?;

    Ok(Icon {
        view,
        size: icon_size,
        categories,
        component_name: feature_name.clone(), // TODO: Make clear why
        feature_name,
    })
}
