use std::{io, path::PathBuf};

use anyhow::Result;
use heck::{ToShoutySnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use snafu::{prelude::*, Backtrace};
use tokio::io::AsyncWriteExt;
use tracing::{error, trace};

use crate::{icon::SvgIcon, icon_library::IconLibrary, main_library::MainLibrary};

#[derive(Debug, Snafu)]
pub(crate) enum Error {
    #[snafu(display("Unable to create file {path:?}"))]
    CreateFile {
        source: io::Error,
        path: PathBuf,
        backtrace: Backtrace,
    },
    #[snafu(display("Unable to parse TokenStream"))]
    ParseTokenStream {
        source: syn::Error,
        backtrace: Backtrace,
    },
}

#[derive(Debug)]
pub(crate) struct LibRs<T> {
    pub path: PathBuf,
    pub(crate) _phantom: std::marker::PhantomData<T>,
}

impl<T: std::fmt::Debug> LibRs<T> {
    pub(crate) async fn init(&self) -> Result<(), Error> {
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
    async fn append(&self) -> Result<tokio::io::BufWriter<tokio::fs::File>> {
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

    async fn write(&self, content: String) -> Result<()> {
        let mut writer = self.append().await?;
        writer.write_all("\n".as_bytes()).await?;
        writer.write_all(content.as_bytes()).await?;
        writer.flush().await.map_err(|err| {
            error!(?err, "Could not flush lib.rs file after writing.");
            err
        })?;
        Ok(())
    }
}

impl LibRs<MainLibrary> {
    pub async fn write_lib_rs(
        &self,
        component_name: &str,
        enum_name: &str,
        icon_libs: &[IconLibrary],
    ) -> Result<()> {
        let reexports = Self::build_reexports(icon_libs)?;
        let enum_code = Self::build_enum(enum_name, icon_libs)?;
        let component = Self::build_component(component_name, enum_name, icon_libs)?;
        self.write(reexports).await?;
        self.write(enum_code).await?;
        self.write(component).await?;

        Ok(())
    }

    fn build_reexports(icon_libs: &[IconLibrary]) -> Result<String> {
        let statements = icon_libs.iter().map(|lib| {
            let lib_short_name = &lib.package.meta.short_name;
            let short_name_upper = lib_short_name.to_upper_camel_case();
            let lib_name_ident = format_ident!("icondata_{}", lib_short_name);
            quote! {
                #[cfg(feature = #short_name_upper)]
                pub use #lib_name_ident::*;
            }
        });
        let statements = quote! {
            #(#statements)*
        };
        let tokens_file = syn::parse2::<syn::File>(statements).context(ParseTokenStreamSnafu {})?;
        Ok(prettyplease::unparse(&tokens_file))
    }

    fn build_enum(enum_name: &str, icon_libs: &[IconLibrary]) -> Result<String> {
        let variants = icon_libs.iter().map(|lib| {
            let short_name_upper = &lib.package.meta.short_name.to_upper_camel_case();
            let short_name_upper_ident = Ident::new(&short_name_upper, Span::call_site());
            let lib_name_ident = format_ident!("icondata_{}", &lib.package.meta.short_name);
            let lib_enum_ident = Ident::new(&lib.enum_name(), Span::call_site());
            quote! {
                #[cfg(feature = #short_name_upper)]
                #short_name_upper_ident(#lib_name_ident::#lib_enum_ident)
            }
        });

        let enum_ident = Ident::new(enum_name, Span::call_site());

        let icon_enum = quote! {
            #[cfg_attr(feature = "serde", derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, serde::Serialize, serde::Deserialize))]
            #[cfg_attr(not(feature = "serde"), derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy))]
            pub enum #enum_ident {
                #(#variants),*
            }
        };

        let data_impl_match_arms = icon_libs.iter().map(|lib| {
            let short_name_upper = &lib.package.meta.short_name.to_upper_camel_case();
            let lib_short_name_ident = Ident::new(&short_name_upper, Span::call_site());
            quote! {
                #[cfg(feature = #short_name_upper)]
                Self::#lib_short_name_ident(icon) => icondata_core::IconData::data(icon)
            }
        });

        let data_impl = quote! {
            impl<'a> icondata_core::IconData<'a> for crate::#enum_ident {
                fn data(self) -> &'a icondata_core::Data {
                    match self {
                        #(#data_impl_match_arms),*
                    }
                }
            }
        };

        let from_impls = icon_libs
            .iter()
            .map(|lib| {
                let short_name_upper = &lib.package.meta.short_name.to_upper_camel_case();
                let lib_short_name_ident = Ident::new(&short_name_upper, Span::call_site());
                let lib_enum_ident = Ident::new(&lib.enum_name(), Span::call_site());
                quote! {
                    #[cfg(feature = #short_name_upper)]
                    impl From<#lib_enum_ident> for #enum_ident {
                        fn from(value: #lib_enum_ident) -> Self {
                            Self::#lib_short_name_ident(value)
                        }
                    }

                }
            })
            .collect::<Vec<_>>();

        let code = quote! {
            #icon_enum

            #data_impl

            #(#from_impls)*
        };

        let tokens_file = syn::parse2::<syn::File>(code).context(ParseTokenStreamSnafu {})?;
        Ok(prettyplease::unparse(&tokens_file))
    }

    fn build_component(
        component_name: &str,
        enum_name: &str,
        icon_libs: &[IconLibrary],
    ) -> Result<String> {
        let component_ident = Ident::new(component_name, Span::call_site());
        let enum_ident = Ident::new(enum_name, Span::call_site());

        let features = icon_libs.iter().map(|lib| {
            let lib_short_name = &lib.package.meta.short_name.to_upper_camel_case();
            quote! {
                feature = #lib_short_name,
            }
        });

        let component = quote! {
            #[cfg(any(
                #(#features)*
            ))]
            #[leptos::component]
            pub fn #component_ident(
                cx: leptos::Scope,
                /// The icon to show.
                #[prop(into)]
                icon: crate::#enum_ident,
                /// The width of the icon (horizontal side length of the square surrounding the icon). Defaults to "1em".
                #[prop(into, optional)]
                width: Option<String>,
                /// The height of the icon (vertical side length of the square surrounding the icon). Defaults to "1em".
                #[prop(into, optional)]
                height: Option<String>,
                /// HTML class attribute.
                #[prop(into, optional)]
                class: Option<String>,
                /// HTML style attribute.
                #[prop(into, optional)]
                style: Option<String>,
            ) -> impl leptos::IntoView {
                let data = icondata_core::IconData::data(icon);
                let mut svg = leptos::svg::svg(cx);
                if let Some(classes) = class {
                    svg = svg.classes(classes);
                }
                svg = match (style, data.style) {
                    (Some(a), Some(b)) => svg.attr("style", format!("{a} {b}")),
                    (Some(a), None) => svg.attr("style", a),
                    (None, Some(b)) => svg.attr("style", b),
                    (None, None) => svg,
                };
                if let Some(x) = data.x {
                    svg = svg.attr("x", x);
                }
                if let Some(y) = data.y {
                    svg = svg.attr("x", y);
                }
                svg = svg.attr("width", leptos::Attribute::String(match (width, data.width) {
                    (Some(a), Some(_b)) => a,
                    (Some(a), None) => a,
                    (None, Some(_b)) => "1em".to_owned(),
                    (None, None) => "1em".to_owned(),
                }));
                svg = svg.attr("height", leptos::Attribute::String(match (height, data.height) {
                    (Some(a), Some(_b)) => a,
                    (Some(a), None) => a,
                    (None, Some(_b)) => "1em".to_owned(),
                    (None, None) => "1em".to_owned(),
                }));
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
                svg = svg.attr("role", "graphics-symbol");
                svg = svg.inner_html(data.data);
                leptos::IntoView::into_view(svg, cx)
            }
        };

        let tokens_file = syn::parse2::<syn::File>(component).context(ParseTokenStreamSnafu {})?;
        Ok(prettyplease::unparse(&tokens_file))
    }
}

impl LibRs<IconLibrary> {
    pub(crate) async fn write_lib_rs(&self, enum_name: &str, icons: &[SvgIcon]) -> Result<()> {
        let enum_code = Self::build_enum(enum_name, icons)?;
        let icon_data = Self::build_icon_data(enum_name, icons)?;
        self.write(enum_code).await?;
        self.write(icon_data).await?;

        Ok(())
    }

    fn build_enum(enum_name: &str, icons: &[SvgIcon]) -> Result<String> {
        let variants = icons.iter().map(|icon| {
            let feature_name = &icon.feature.name;
            let feature_ident = Ident::new(feature_name.as_str(), Span::call_site());
            quote! {
                #[cfg(feature = #feature_name)]
                #feature_ident
            }
        });

        let enum_ident = Ident::new(enum_name, Span::call_site());

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

    fn build_icon_data(enum_name: &str, icons: &[SvgIcon]) -> Result<String> {
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
        let icons = icons
            .iter()
            .map(|icon| Data {
                style: icon
                    .svg
                    .svg_attributes
                    .style
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                x: icon
                    .svg
                    .svg_attributes
                    .x
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                y: icon
                    .svg
                    .svg_attributes
                    .y
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                width: icon
                    .svg
                    .svg_attributes
                    .width
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                height: icon
                    .svg
                    .svg_attributes
                    .height
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                view_box: icon
                    .svg
                    .svg_attributes
                    .view_box
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                stroke_linecap: icon
                    .svg
                    .svg_attributes
                    .stroke_linecap
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                stroke_linejoin: icon
                    .svg
                    .svg_attributes
                    .stroke_linejoin
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                stroke_width: icon
                    .svg
                    .svg_attributes
                    .stroke_width
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                stroke: icon
                    .svg
                    .svg_attributes
                    .stroke
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                fill: icon
                    .svg
                    .svg_attributes
                    .fill
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                data: icon.svg.content.clone(),
                feature_name: icon.feature.name.to_owned(),
            })
            .collect::<Vec<_>>();

        let const_icon_data = icons.iter().map(|icon| {
            let feature_name = &icon.feature_name;
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
                const #const_data_ident: icondata_core::Data = icondata_core::Data {
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

            impl<'a> icondata_core::IconData<'a> for #enum_ident {
                fn data(self) -> &'a icondata_core::Data {
                    match self {
                        #(#match_arms),*
                    }
                }
            }
        };

        let tokens_file: syn::File = syn::parse2(enum_impl)?;
        Ok(prettyplease::unparse(&tokens_file))
    }
}
