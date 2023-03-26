use std::{path::PathBuf, process::Command};

use anyhow::{anyhow, Ok, Result};
use tracing::{info, instrument, warn};

use crate::{
    package::{self, GitTarget, PackageMetadata},
    path,
};

#[instrument(level = "info", fields(package = meta.package_name.as_ref()))]
pub(crate) async fn remove_package(meta: &PackageMetadata) -> Result<()> {
    let package_download_path = path::download_path(meta.download_dir.as_ref());
    if package_download_path.exists() {
        info!(
            ?package_download_path,
            "Removing existing download folder for package"
        );
        tokio::fs::remove_dir_all(&package_download_path).await?;
    }
    Ok(())
}

#[instrument(level = "info")]
pub(crate) fn download_package(meta: &PackageMetadata) -> Result<()> {
    let target_dir = path::download_path(meta.download_dir.as_ref());
    info!(?target_dir, "Constructed target directory for download.");

    match &meta.source {
        package::PackageSource::Git { url, target } => {
            if target_dir.exists() {
                info!(
                    ?target_dir,
                    "Download target directory already exists. Assuming git repository."
                );
                perform_checkout(target, &target_dir)
            } else {
                info!(
                    ?target_dir,
                    "Download target directory does not exist. Cloning the repository."
                );
                perform_direct_clone(url, target, &target_dir).or_else(|_err| {
                    info!("Direct clone unsuccessful. Trying clone with checkout...");
                    perform_clone_without_checkout(url, &target_dir)?;
                    perform_checkout(target, &target_dir)
                })
            }
        }
    }
}

/// Clone the given repository at a specific commit ref or tag.
fn perform_direct_clone(git_url: &str, git_target: &GitTarget, target_dir: &PathBuf) -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.args(["clone", "--depth", "1", "--branch"]);

    match git_target {
        package::GitTarget::Branch {
            name: _,
            commit_ref,
            version_hint: _,
        } => {
            // NOTE: The hash alone is enough here. We do not also need to specify a branch!
            cmd.arg(commit_ref.as_ref());
        }
        package::GitTarget::Tag { name, version: _ } => {
            // NOTE: The tag-name alone is enough here. We do not need to prepend it with "tags/"!
            //&format!("tags/{}", name.as_ref());
            cmd.arg(name.as_ref());
        }
    }

    // Adding the repository url.
    cmd.arg(git_url);

    // Adding the destination path.
    cmd.arg(target_dir);

    info!(
        ?cmd,
        "Constructed command to directly clone GIT repository."
    );

    let output = cmd
        .current_dir(path::build_crate(""))
        .output()
        .map_err(anyhow::Error::from)?;

    if output.status.success() {
        info!("Direct clone successful.");
        Ok(())
    } else {
        let std_out = String::from_utf8_lossy(&output.stdout);
        let std_err = String::from_utf8_lossy(&output.stderr);
        warn!(
            exit_code = output.status.code(),
            ?std_out,
            ?std_err,
            "Direct clone failed."
        );
        Err(anyhow!("asd"))
    }
}

/// Clone the given repository at a specific commit ref or tag.
fn perform_clone_without_checkout(git_url: &str, target_dir: &PathBuf) -> Result<()> {
    let clone_output = {
        let mut cmd = Command::new("git");
        cmd.args(["clone", "--depth", "1", "--no-checkout", git_url]);
        cmd.arg(target_dir);

        info!(
            ?cmd,
            "Constructed command to clone GIT repository using --no-checkout strategy."
        );

        cmd.current_dir(path::build_crate(""))
            .output()
            .map_err(anyhow::Error::from)
    }?;

    if !clone_output.status.success() {
        let std_out = String::from_utf8_lossy(&clone_output.stdout);
        let std_err = String::from_utf8_lossy(&clone_output.stderr);
        warn!(
            exit_code = clone_output.status.code(),
            ?std_out,
            ?std_err,
            "Clone failed."
        );
        return Err(anyhow!("Clone failed."));
    }
    info!("Clone with --no-checkout successful.");

    Ok(())
}

fn perform_checkout(git_target: &GitTarget, target_dir: &PathBuf) -> Result<()> {
    let checkout_output = {
        let mut cmd = Command::new("git");
        cmd.args([
            "checkout",
            match git_target {
                package::GitTarget::Branch {
                    name: _,
                    commit_ref,
                    version_hint: _,
                } => commit_ref.as_ref(),
                package::GitTarget::Tag { name, version: _ } => name.as_ref(),
            },
        ]);
        cmd.arg(target_dir);

        info!(?cmd, "Constructed command to checkout specific branch.");

        cmd.current_dir(target_dir)
            .output()
            .map_err(anyhow::Error::from)
    }?;

    if !checkout_output.status.success() {
        let std_out = String::from_utf8_lossy(&checkout_output.stdout);
        let std_err = String::from_utf8_lossy(&checkout_output.stderr);
        warn!(
            exit_code = checkout_output.status.code(),
            ?std_out,
            ?std_err,
            "Checkout failed."
        );
        return Err(anyhow!("Checkout failed."));
    }
    info!("Checkout successful.");

    Ok(())
}
