use std::{io, path::PathBuf};

use anyhow::Result;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use snafu::{prelude::*, Backtrace};
use tokio::io::AsyncWriteExt;
use tracing::{error, trace};
use heck::ToShoutySnakeCase;

use crate::icon::SvgIcon;

#[derive(Debug, Snafu)]
pub(crate) enum Error {
    #[snafu(display("Unable to create file {path:?}"))]
    CreateFile {
        source: io::Error,
        path: PathBuf,
        backtrace: Backtrace,
    },
}

#[derive(Debug)]
pub(crate) struct LibRs {
    pub path: PathBuf,
}

impl LibRs {
    pub async fn init(&self) -> Result<(), Error> {
        trace!(path = ?self.path, "Creating new lib.rs file.");
        tokio::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&self.path)
            .await
            .with_context(|_| CreateFileSnafu {
                path: self.path.clone(),
            })?;
        Ok(())
    }

    /// Opens the file for appending thereby creating it if non-existent.
    pub async fn append(&self) -> Result<tokio::io::BufWriter<tokio::fs::File>> {
        Ok(tokio::io::BufWriter::new(
            tokio::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.path)
                .await
                .map_err(|err| {
                    error!(?err, "Could not open lib.rs file to append modules.");
                    err
                })?,
        ))
    }

    pub fn build_enum(name: &str, icons: &[SvgIcon]) -> Result<String> {
        let variants = icons.iter().map(|icon| {
            let feature_name = &icon.feature.name;
            let feature_ident = Ident::new(feature_name.as_str(), Span::call_site());
            quote! {
                #[cfg(feature = #feature_name)]
                #feature_ident
            }
        });

        let enum_ident = Ident::new(name, Span::call_site());

        let icon_enum = quote! {
            #[cfg_attr(feature = "serde", derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, serde::Serialize, serde::Deserialize))]
            #[cfg_attr(not(feature = "serde"), derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy))]
            pub enum #enum_ident {
                #(#variants),*
            }
        };

        let tokens_file: syn::File = syn::parse2(icon_enum).map_err(|err| {
            error!(?err, "Error parsing enum to token stream");
            err
        })?;
        Ok(prettyplease::unparse(&tokens_file))
    }

    pub async fn write_enum(&mut self, enum_code: String) -> Result<()> {
        let mut writer = self.append().await?;
        writer.write_all("\n".as_bytes()).await?;
        writer.write_all(enum_code.as_bytes()).await?;
        writer.flush().await.map_err(|err| {
            error!(?err, "Could not flush lib.rs file after writing.");
            err
        })?;
        Ok(())
    }

    pub fn build_icon_component(
        enum_name: &str,
        icons: &[SvgIcon],
    ) -> Result<String> {
        struct Data {
            style: Option<String>,
            x: Option<String>,
            y: Option<String>,
            width: Option<String>,
            height: Option<String>,
            view_box: Option<String>,
            stroke_linecap: Option<String>,
            stroke_linejoin: Option<String>,
            stroke_width: Option<String>,
            stroke: Option<String>,
            fill: Option<String>,
            data: String,
            feature_name: String,
        }
        let icons = icons.iter().map(|icon| Data {
            style: icon.svg.svg_attributes.style.as_ref().map(|it| it.value.to_owned()),
            x: icon.svg.svg_attributes.x.as_ref().map(|it| it.value.to_owned()),
            y: icon.svg.svg_attributes.y.as_ref().map(|it| it.value.to_owned()),
            width: icon.svg.svg_attributes.width.as_ref().map(|it| it.value.to_owned()),
            height: icon.svg.svg_attributes.height.as_ref().map(|it| it.value.to_owned()),
            view_box: icon.svg.svg_attributes.view_box.as_ref().map(|it| it.value.to_owned()),
            stroke_linecap: icon.svg.svg_attributes.stroke_linecap.as_ref().map(|it| it.value.to_owned()),
            stroke_linejoin: icon.svg.svg_attributes.stroke_linejoin.as_ref().map(|it| it.value.to_owned()),
            stroke_width: icon.svg.svg_attributes.stroke_width.as_ref().map(|it| it.value.to_owned()),
            stroke: icon.svg.svg_attributes.stroke.as_ref().map(|it| it.value.to_owned()),
            fill: icon.svg.svg_attributes.fill.as_ref().map(|it| it.value.to_owned()),
            data: icon.svg.content.clone(),
            feature_name: icon.feature.name.to_owned(),
        }).collect::<Vec<_>>();

        let const_icon_data = icons.iter().map(|icon| {
            let feature_name = &icon.feature_name;
            let feature_ident = Ident::new(feature_name.as_str(), Span::call_site());
            let enum_ident = Ident::new(enum_name, Span::call_site());
            let const_data_name = feature_name.to_shouty_snake_case();
            let const_data_ident = Ident::new(&const_data_name, Span::call_site());

            fn quote_opt(value: &Option<String>) -> TokenStream {
                match value {
                    Some(value) => quote! { Some(#value) },
                    None => quote! { None },
                }
            }

            let style = quote_opt(&icon.style);
            let x = quote_opt(&icon.x);
            let y = quote_opt(&icon.y);
            let width = quote_opt(&icon.width);
            let height = quote_opt(&icon.height);
            let view_box = quote_opt(&icon.view_box);
            let stroke_linecap = quote_opt(&icon.stroke_linecap);
            let stroke_linejoin = quote_opt(&icon.stroke_linejoin);
            let stroke_width = quote_opt(&icon.stroke_width);
            let stroke = quote_opt(&icon.stroke);
            let fill = quote_opt(&icon.fill);
            let data = &icon.data;

            quote! {
                #[cfg(feature = #feature_name)]
                const #const_data_ident: leptos_icons_core::Data = leptos_icons_core::Data {
                    style: #style,
                    x: #x,
                    y: #y,
                    width: #width,
                    height: #height,
                    view_box: #view_box,
                    stroke_linecap: #stroke_linecap,
                    stroke_linejoin: #stroke_linejoin,
                    stroke_width: #stroke_width,
                    stroke: #stroke,
                    fill: #fill,
                    data: #data,
                };
            }
        });

        let match_arms = icons.iter().map(|icon| {
            let feature_name = &icon.feature_name;
            let feature_ident = Ident::new(feature_name.as_str(), Span::call_site());
            let enum_ident = Ident::new(enum_name, Span::call_site());
            let const_data_name = feature_name.to_shouty_snake_case();
            let const_data_ident = Ident::new(&const_data_name, Span::call_site());

            quote! {
                #[cfg(feature = #feature_name)]
                #enum_ident::#feature_ident => &#const_data_ident
            }
        });

        let enum_ident = Ident::new(enum_name, Span::call_site());

        let enum_impl = quote! {
            #(#const_icon_data)*

            impl<'a> leptos_icons_core::IconData<'a> for #enum_ident {
                fn data(self) -> &'a leptos_icons_core::Data {
                    match self {
                        #(#match_arms),*
                    }
                }
            }
        };

        let tokens_file: syn::File = syn::parse2(enum_impl)?;
        Ok(prettyplease::unparse(&tokens_file))
    }

    pub async fn write_component(&mut self, component_code: String) -> Result<()> {
        let mut writer = self.append().await?;
        writer.write_all("\n".as_bytes()).await?;
        writer.write_all(component_code.as_bytes()).await?;
        writer.flush().await.map_err(|err| {
            error!(?err, "Could not flush lib.rs file after writing.");
            err
        })?;
        Ok(())
    }
}
