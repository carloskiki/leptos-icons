use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SemVer {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub prerelease: Option<String>,
    pub build: Option<String>,
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
