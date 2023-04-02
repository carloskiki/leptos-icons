use std::{io, path::PathBuf};

use anyhow::Result;
use heck::{ToUpperCamelCase, ToSnakeCase};
use proc_macro2::{Ident, Span};
use quote::quote;
use snafu::{prelude::*, Backtrace};
use tokio::io::AsyncWriteExt;
use tracing::{error, trace};

use crate::library::IconLibrary;

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

    pub fn build_enum(name: &str, icon_libs: &[IconLibrary]) -> Result<String> {
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

        let enum_ident = Ident::new(name, Span::call_site());

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
}
