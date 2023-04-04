use std::{io, path::PathBuf};

use anyhow::Result;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use snafu::{prelude::*, Backtrace};
use tokio::io::AsyncWriteExt;
use tracing::{error, trace};

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
        component_name: &str,
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

        let data_struct = quote! {
            struct Data {
                style: Option<&'static str>,
                x: Option<&'static str>,
                y: Option<&'static str>,
                width: Option<&'static str>,
                height: Option<&'static str>,
                view_box: Option<&'static str>,
                stroke_linecap: Option<&'static str>,
                stroke_linejoin: Option<&'static str>,
                stroke_width: Option<&'static str>,
                stroke: Option<&'static str>,
                fill: Option<&'static str>,
                data: &'static str,
            }
        };

        let match_arms = icons.iter().map(|icon| {
            let feature_name = &icon.feature_name;
            let feature_ident = Ident::new(feature_name.as_str(), Span::call_site());
            let enum_ident = Ident::new(enum_name, Span::call_site());

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
                #enum_ident::#feature_ident => Data {
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
                }
            }
        });

        let component_ident = Ident::new(component_name, Span::call_site());
        let enum_ident = Ident::new(enum_name, Span::call_site());

        let data_struct_impl = quote! {
            impl Data {
                pub fn of(icon: #enum_ident) -> Data {
                    match icon {
                        #(#match_arms),*
                    }
                }
            }
        };

        // Note: All parameters are `#[allow(unused)]` as the match may not include any arms if a user never activated any icon features, leading to unused warnings.
        // Note: The title prop is `unwrap_or_else`ed in each individual match arm!
        let tokens = quote! {
            #data_struct

            #data_struct_impl

            #[allow(non_snake_case)]
            pub fn #component_ident(
                cx: leptos::Scope,
                // Variant of the icon to display.
                icon: #enum_ident,
                // The width of the icon (horizontal side length of the square surrounding the icon). Defaults to "1em".
                width: Option<String>,
                // The height of the icon (vertical side length of the square surrounding the icon). Defaults to "1em".
                height: Option<String>,
                // HTML class attribute.
                class: Option<String>,
                // HTML style attribute.
                style: Option<String>,
            ) -> leptos::View {
                let data = Data::of(icon);

                let mut svg = leptos::svg::svg(cx);

                if let Some(classes) = class {
                    svg = svg.classes(classes);
                }

                svg = match (style, data.style) {
                    (Some(a), Some(b)) => {svg.attr("style", format!("{a} {b}"))},
                    (Some(a), None) => {svg.attr("style", a)},
                    (None, Some(b)) => {svg.attr("style", b)},
                    (None, None) => {svg},
                };

                if let Some(x) = data.x {
                    svg = svg.attr("x", x);
                }
                if let Some(y) = data.y {
                    svg = svg.attr("x", y);
                }

                svg = match (width, data.width) {
                    (Some(a), Some(_b)) => {svg.attr("width", a)},
                    (Some(a), None) => {svg.attr("width", a)},
                    (None, Some(_b)) => {svg.attr("width", "1em")},
                    (None, None) => {svg.attr("width", "1em")},
                };

                svg = match (height, data.height) {
                    (Some(a), Some(_b)) => {svg.attr("height", a)},
                    (Some(a), None) => {svg.attr("height", a)},
                    (None, Some(_b)) => {svg.attr("height", "1em")},
                    (None, None) => {svg.attr("height", "1em")},
                };

                if let Some(view_box) = data.view_box {
                    svg = svg.attr("viewBox", view_box);
                }

                if let Some(stroke_linecap) = data.stroke_linecap {
                    svg = svg.attr("stroke-linecap", stroke_linecap);
                }
                if let Some(stroke_linejoin) = data.stroke_linejoin {
                    svg = svg.attr("stroke-linejoin", stroke_linejoin);
                }
                if let Some(stroke_width) = data.stroke_width {
                    svg = svg.attr("stroke-width", stroke_width);
                }
                if let Some(stroke) = data.stroke {
                    svg = svg.attr("stroke", stroke);
                }
                svg = svg.attr("fill", data.fill.unwrap_or("currentColor"));
                svg = svg.attr("role", "graphics-symbol"); // Ignoring the data property...

                svg = svg.inner_html(data.data);

                leptos::IntoView::into_view(svg, cx)
            }
        };

        let tokens_file: syn::File = syn::parse2(tokens)?;
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
