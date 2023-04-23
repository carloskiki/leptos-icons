use std::{
    io,
    path::PathBuf,
    process::{Command, ExitStatus},
};

use snafu::{prelude::*, Backtrace};
use tracing::{debug, info, instrument, warn};

use crate::{
    package::{self, GitTarget},
    path,
};

#[derive(Debug, Snafu)]
pub(crate) enum Error {
    #[snafu(display("Could not execute command {cmd}"))]
    ExecCommand {
        source: io::Error,
        cmd: String,
        backtrace: Backtrace,
    },
    #[snafu(display("Command exited with error. status_code: {exit_status}, std_out:{std_out}, std_err:{std_err}"))]
    Command {
        exit_status: ExitStatus,
        std_out: String,
        std_err: String,
        backtrace: Backtrace,
    },
}

/// Clone the given repository at a specific commit ref or tag.
#[instrument(level = "info")]
pub(crate) fn clone(
    git_url: &str,
    git_target: &GitTarget,
    target_dir: &PathBuf,
) -> Result<(), Error> {
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

    debug!(
        ?cmd,
        "Constructed command to directly clone GIT repository."
    );

    let output = cmd
        .current_dir(path::build_crate(""))
        .output()
        .context(ExecCommandSnafu {
            cmd: format!("{cmd:?}"),
        })?;

    ensure!(output.status.success(), {
        let std_out = String::from_utf8_lossy(&output.stdout).into_owned();
        let std_err = String::from_utf8_lossy(&output.stderr).into_owned();
        warn!(
            exit_code = output.status.code(),
            ?std_out,
            ?std_err,
            "Direct clone failed."
        );
        CommandSnafu {
            exit_status: output.status,
            std_out,
            std_err,
        }
    });

    info!("Direct clone successful.");
    Ok(())
}

/// Clone the given repository without checking out any specific commit or tag.
/// This might be used in conjunction with `git::checkout()` if a simple `git::clone()` fails.
/// You have to call `git::checkout()` after this was successful.
#[instrument(level = "info")]
pub(crate) fn clone_without_checkout(git_url: &str, target_dir: &PathBuf) -> Result<(), Error> {
    let clone_output = {
        let mut cmd = Command::new("git");
        cmd.args(["clone", "--depth", "1", "--no-checkout", git_url]);
        cmd.arg(target_dir);
        debug!(
            ?cmd,
            "Constructed command to clone GIT repository using --no-checkout strategy."
        );
        cmd.current_dir(path::build_crate(""))
            .output()
            .context(ExecCommandSnafu {
                cmd: format!("{cmd:?}"),
            })
    }?;

    ensure!(clone_output.status.success(), {
        let std_out = String::from_utf8_lossy(&clone_output.stdout).into_owned();
        let std_err = String::from_utf8_lossy(&clone_output.stderr).into_owned();
        warn!(
            exit_code = clone_output.status.code(),
            ?std_out,
            ?std_err,
            "Clone failed."
        );
        CommandSnafu {
            exit_status: clone_output.status,
            std_out,
            std_err,
        }
    });

    info!("Clone with --no-checkout successful.");
    Ok(())
}

/// Checkout a specific tag or commit. `target_dir` must point to a directory containing a cloned git repository.v
#[instrument(level = "info")]
pub(crate) fn checkout(git_target: &GitTarget, target_dir: &PathBuf) -> Result<(), Error> {
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
        debug!(?cmd, "Constructed command to checkout specific branch.");
        cmd.current_dir(target_dir)
            .output()
            .context(ExecCommandSnafu {
                cmd: format!("{cmd:?}"),
            })
    }?;

    ensure!(checkout_output.status.success(), {
        let std_out = String::from_utf8_lossy(&checkout_output.stdout).into_owned();
        let std_err = String::from_utf8_lossy(&checkout_output.stderr).into_owned();
        warn!(
            exit_code = checkout_output.status.code(),
            ?std_out,
            ?std_err,
            "Checkout failed."
        );
        CommandSnafu {
            exit_status: checkout_output.status,
            std_out,
            std_err,
        }
    });

    info!("Checkout successful.");
    Ok(())
}
