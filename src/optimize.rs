use std::process::Command;
use std::str::from_utf8;

use anyhow::{anyhow, Result};
use svgcleaner::cleaner::{clean_doc, parse_data, write_buffer};
use svgcleaner::{CleaningOptions, ParseOptions, WriteOptions};

use crate::types::{IconName, IconPackage};

pub(crate) fn optimize(icon_content: String) -> Result<String> {
    let optimization_output = Command::new("svgo").arg("-s").arg(icon_content).output()?;
    Ok(from_utf8(&optimization_output.stdout)?.to_owned())
}
