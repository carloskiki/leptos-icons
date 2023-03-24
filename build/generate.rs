use anyhow::{anyhow, Result};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::{
    fs::{create_dir, create_dir_all, File, OpenOptions},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};
use xml::attribute::OwnedAttribute;

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
    let icon_file_content = icon_file_content(
        &icon_feature_name,
        &icon_component_name,
        &icon.content,
        &icon.attributes,
    );
    icon_file.write_all(icon_file_content?.as_bytes())?;

    icon_path.set_extension("rs");
    let mut upper_module_file = OpenOptions::new().append(true).open(&icon_path)?;
    declare_mod(&mut upper_module_file, &icon_component_name)?;

    cargo_file.write_all(format!("{} = []\n", &icon_feature_name).as_bytes())?;

    Ok(())
}

fn icon_file_content(
    icon_feature_name: &str,
    icon_component_name: &str,
    icon_content: &str,
    attributes: &Vec<OwnedAttribute>,
) -> Result<String> {
    let doc_comment =
        format!("This icon requires the feature `{icon_feature_name}` to be enabled.");
    let attributes = attributes_token_stream(attributes)?;
    let icon_component_ident = Ident::new(icon_component_name, Span::call_site());
    let icon_content_ident: TokenStream = icon_content
        .parse()
        .map_err(|_| anyhow!("could not transform icon_content to ident"))?;

    let tokens = quote! {
        #[cfg(feature = #icon_feature_name)]
        use leptos::*;

        #[cfg(feature = #icon_feature_name)]
        #[doc = #doc_comment]
        #[component]
        pub fn #icon_component_ident(
            cx: Scope,
            /// The size of the icon (The side length of the square surrounding the icon).
            /// Defaults to "1em".
            #[prop(into)] #[prop(optional)] size: String,
            /// HTML class attribute.
            #[prop(into)] #[prop(optional)] class: String,
            /// Color of the icon.
            /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
            #[prop(into)] #[prop(optional)] color: String,
            /// HTML style attribute.
            #[prop(into)] #[prop(optional)] style: String,
            /// Accessibility title.
            #[prop(into)] #[prop(optional)] title: String,
            ) -> impl IntoView {
            view! { cx,
            <svg
            class=class
            stroke="currentColor"
            fill="currentColor"
            stroke_witdh="0"
            style=style
            #attributes
            width={size.clone()}
            height={size}
            >
                #icon_content_ident
                <title>{title}</title>
            </svg>
            }
        }
    };

    let tokens_file: syn::File = syn::parse2(tokens)?;
    Ok(prettyplease::unparse(&tokens_file))
}

fn attributes_token_stream(attributes: &Vec<OwnedAttribute>) -> Result<TokenStream> {
    attributes
        .into_iter()
        .map(|attribute| {
            let attribute_val = &attribute.value;
            let attr_ident: TokenStream = attribute
                .name
                .local_name
                .parse()
                .map_err(|_| anyhow!("could not convert attributes to token stream"))?;
            Ok(quote!(#attr_ident=#attribute_val))
        })
        .collect::<Result<TokenStream>>()
}
