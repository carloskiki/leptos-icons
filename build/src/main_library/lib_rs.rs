use std::{io, path::PathBuf};

use anyhow::Result;
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, Span};
use quote::quote;
use snafu::{prelude::*, Backtrace};
use tokio::io::AsyncWriteExt;
use tracing::{error, trace};

use crate::icon_library::IconLibrary;

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

    pub fn build_reexports(icon_libs: &[IconLibrary]) -> Result<String> {
        let statements = icon_libs.iter().map(|lib| {
            let lib_name_ident = Ident::new(&lib.name.to_snake_case(), Span::call_site());
            quote! {
                pub use #lib_name_ident::*;
            }
        });
        let statements = quote! {
            #(#statements)*
        };
        let tokens_file = syn::parse2::<syn::File>(statements).context(ParseTokenStreamSnafu {})?;
        Ok(prettyplease::unparse(&tokens_file))
    }

    pub fn build_enum(enum_name: &str, icon_libs: &[IconLibrary]) -> Result<String> {
        let variants = icon_libs.iter().map(|lib| {
            let lib_short_name_ident = Ident::new(
                &lib.package.meta.short_name.to_upper_camel_case(),
                Span::call_site(),
            );
            let lib_name_ident = Ident::new(&lib.name.to_snake_case(), Span::call_site());
            let lib_enum_ident = Ident::new(&lib.enum_name(), Span::call_site());
            quote! {
                #lib_short_name_ident(#lib_name_ident::#lib_enum_ident)
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

        let tokens_file = syn::parse2::<syn::File>(icon_enum).context(ParseTokenStreamSnafu {})?;
        Ok(prettyplease::unparse(&tokens_file))
    }

    pub fn build_component(
        component_name: &str,
        enum_name: &str,
        icon_libs: &[IconLibrary],
    ) -> Result<String> {
        let component_ident = Ident::new(component_name, Span::call_site());
        let enum_ident = Ident::new(enum_name, Span::call_site());

        let match_arms = icon_libs.iter().map(|lib| {
            let lib_short_name_ident = Ident::new(
                &lib.package.meta.short_name.to_upper_camel_case(),
                Span::call_site(),
            );
            let lib_component_ident = Ident::new(&lib.component_name(), Span::call_site());
            quote! {
                #[allow(unreachable_code, unreachable_patterns)]
                #enum_ident::#lib_short_name_ident(icon) => view! { cx,
                    <#lib_component_ident icon width height class style title/>
                }
                .into_view(cx)
            }
        });

        let component = quote! {
            use leptos::*;

            #[component]
            pub fn #component_ident(
                cx: Scope,

                icon: #enum_ident,

                /// The width of the icon (horizontal side length of the square surrounding the icon). Defaults to "1em".
                #[prop(into, optional)]
                #[allow(unused)]
                width: Option<String>,
                /// The height of the icon (vertical side length of the square surrounding the icon). Defaults to "1em".
                #[prop(into, optional)]
                #[allow(unused)]
                height: Option<String>,
                /// HTML class attribute.
                #[prop(into, optional)]
                #[allow(unused)]
                class: Option<String>,
                /// HTML style attribute.
                #[prop(into, optional)]
                #[allow(unused)]
                style: Option<String>,
                /// ARIA accessibility title.
                #[prop(into, optional)]
                #[allow(unused)]
                title: Option<String>,
            ) -> impl IntoView {
                match icon {
                    #(#match_arms),*
                }
            }
        };

        let tokens_file = syn::parse2::<syn::File>(component).context(ParseTokenStreamSnafu {})?;
        Ok(prettyplease::unparse(&tokens_file))
    }

    pub async fn write(&mut self, content: String) -> Result<()> {
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
