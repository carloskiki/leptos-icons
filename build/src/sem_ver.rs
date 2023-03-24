use std::fmt::Display;

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use snafu::{Backtrace, OptionExt, ResultExt, Snafu};

/// Copied as is form: https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string
/// Captures by name: ...
static SEM_VER_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$"#
    ).expect("Correct regex to be provided. This is a bug found when running tests.")
});

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SemVer {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub prerelease: Option<String>,
    pub build: Option<String>,
}

#[derive(Debug, Snafu)]
pub enum SemVerParseError {
    #[snafu(display(
        "SemVerParseError: Given input did not match the semver-regular-expression."
    ))]
    NoMatch { backtrace: Backtrace },

    #[snafu(display(
        "SemVerParseError: Missing major version information."
    ))]
    MajorVersionMissing { backtrace: Backtrace },

    #[snafu(display(
        "SemVerParseError: Missing minor version information."
    ))]
    MinorVersionMissing { backtrace: Backtrace },

    #[snafu(display(
        "SemVerParseError: Missing patch version information."
    ))]
    PatchVersionMissing { backtrace: Backtrace },

    #[snafu(display(
        "SemVerParseError: Could not parse major version number."
    ))]
    ParseMajorVersion {
        source: std::num::ParseIntError,
        backtrace: Backtrace,
    },

    #[snafu(display(
        "SemVerParseError: Could not parse minor version number."
    ))]
    ParseMinorVersion {
        source: std::num::ParseIntError,
        backtrace: Backtrace,
    },

    #[snafu(display(
        "SemVerParseError: Could not parse patch version number."
    ))]
    ParsePatchVersion {
        source: std::num::ParseIntError,
        backtrace: Backtrace,
    },
}

impl SemVer {
    pub fn parse(sem_ver: &str) -> Result<SemVer, SemVerParseError> {
        let caps = SEM_VER_RE.captures(sem_ver).context(NoMatchSnafu {})?;
        Ok(Self {
            major: caps
                .name("major")
                .context(MajorVersionMissingSnafu {})?
                .as_str()
                .parse::<u32>()
                .context(ParseMajorVersionSnafu {})?,
            minor: caps
                .name("minor")
                .context(MinorVersionMissingSnafu {})?
                .as_str()
                .parse::<u32>()
                .context(ParseMinorVersionSnafu {})?,
            patch: caps
                .name("patch")
                .context(PatchVersionMissingSnafu {})?
                .as_str()
                .parse::<u32>()
                .context(ParsePatchVersionSnafu {})?,
            prerelease: caps.name("prerelease").map(|it| it.as_str().to_owned()),
            build: caps.name("buildmetadata").map(|it| it.as_str().to_owned()),
        })
    }
}

impl Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let major = self.major;
        let minor = self.minor;
        let patch = self.patch;
        match (&self.prerelease, &self.build) {
            (None, None) => f.write_fmt(format_args!("{major}.{minor}.{patch}")),
            (None, Some(build)) => f.write_fmt(format_args!("{major}.{minor}.{patch}+{build}")),
            (Some(pre), None) => f.write_fmt(format_args!("{major}.{minor}.{patch}-{pre}")),
            (Some(pre), Some(build)) => {
                f.write_fmt(format_args!("{major}.{minor}.{patch}-{pre}+{build}"))
            }
        }
    }
}
