use std::process::Command;

use anyhow::{anyhow, Ok, Result};
use tracing::{info, instrument, warn};

use crate::{
    package::{self, GitTarget, PackageMetadata},
    path,
};

#[instrument(level = "info")]
pub(crate) fn clear() -> Result<()> {
    let download_path = path::download_path("");
    if download_path.exists() {
        info!(?download_path, "Removing existing downloads folder");
        std::fs::remove_dir_all(&download_path)?;
    }
    Ok(())
}

#[instrument(level = "info")]
pub(crate) fn download_package(meta: &PackageMetadata) -> Result<()> {
    match &meta.source {
        package::PackageSource::Git { url, target } => {
            perform_direct_clone(url, target, meta.download_dir.as_ref()).or_else(|_err| {
                info!("Direct clone unsuccessful. Trying clone with checkout...");
                perform_clone_with_checkout(url, target, meta.download_dir.as_ref())
            })
        }
    }
}

/// Clone the given repository at a specific commit ref or tag.
fn perform_direct_clone(git_url: &str, git_target: &GitTarget, target_dir: &str) -> Result<()> {
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
    cmd.arg(path::download_path(target_dir).as_os_str());

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
fn perform_clone_with_checkout(
    git_url: &str,
    git_target: &GitTarget,
    target_dir: &str,
) -> Result<()> {
    let clone_output = {
        let mut cmd = Command::new("git");
        cmd.args(["clone", "--depth", "1", "--no-checkout", git_url]);
        cmd.arg(path::download_path(target_dir).as_os_str());

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
            "Clone with checkout failed, as the initial clone failed."
        );
        return Err(anyhow!(
            "Clone with checkout failed, as the initial clone failed."
        ));
    }
    info!("Clone with --no-checkout successful.");

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
        cmd.arg(path::download_path(target_dir).as_os_str());

        info!(?cmd, "Constructed command checkout specific branch.");

        cmd.current_dir(path::download_path(target_dir))
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
            "Clone with checkout failed, as the checkout failed."
        );
        return Err(anyhow!(
            "Clone with checkout failed, as the checkout failed."
        ));
    }
    info!("Checkout successful.");

    Ok(())
}
