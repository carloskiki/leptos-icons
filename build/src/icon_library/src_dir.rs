use std::{io, path::PathBuf};

use snafu::{prelude::*, Backtrace};
use tracing::trace;

use super::lib_rs::{self, LibRs};

#[derive(Debug, Snafu)]
pub(crate) enum Error {
    #[snafu(display("Unable to remove src directory {path:?}"))]
    RemoveDir {
        source: io::Error,
        path: PathBuf,
        backtrace: Backtrace,
    },
    #[snafu(display("Unable to create src directory {path:?}"))]
    CreateDir {
        source: io::Error,
        path: PathBuf,
        backtrace: Backtrace,
    },
    #[snafu(display("Unable to initialize lib.rs"))]
    InitLib {
        source: lib_rs::Error,
        backtrace: Backtrace,
    },
}

#[derive(Debug)]
pub(crate) struct SrcDir {
    pub path: PathBuf,
    pub lib_rs: LibRs,
}

impl SrcDir {
    /// Removes and recreates a fresh src directory containing a fresh lib.rs file.
    pub async fn reset(&mut self) -> Result<(), Error> {
        if self.path.exists() {
            trace!(path = ?self.path, "Removing existing src directory");
            tokio::fs::remove_dir_all(&self.path)
                .await
                .with_context(|_| RemoveDirSnafu {
                    path: self.path.clone(),
                })?;
        }

        trace!(path = ?self.path, "Creating new src directory");
        tokio::fs::create_dir(&self.path)
            .await
            .with_context(|_| CreateDirSnafu {
                path: self.path.clone(),
            })?;

        self.lib_rs.init().await.context(InitLibSnafu {})?;
        Ok(())
    }
}
