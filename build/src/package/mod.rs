use anyhow::Result;
use std::{borrow::Cow, marker::PhantomData, path::PathBuf};
use strum::{EnumIter, IntoEnumIterator};
use tracing::{info, instrument};

use crate::{git, icon::SvgIcon, path, sem_ver::SemVer};

mod reader;

/// Name of the directory, relative to the root of this crate, to which all icon packages should be downloaded.
const DOWNLOAD_DIR: &str = "downloads";

#[derive(Debug, Clone)]
pub(crate) struct Package<S> {
    pub ty: PackageType,
    pub meta: PackageMetadata,
    phantom_data: PhantomData<S>,
}

/// It is not guaranteed that the package was downloaded to teh exact version specified.
#[derive(Debug, Clone)]
pub struct Unknown {}

/// The package was successfully downloaded. Icon data can be read.
#[derive(Debug, Clone)]
pub struct Downloaded {}

impl<S> Package<S> {
    pub fn download_path(&self) -> PathBuf {
        path::build_crate(DOWNLOAD_DIR).join(self.meta.download_dir.as_ref())
    }
}

impl Package<Unknown> {
    pub fn all() -> Vec<Package<Unknown>> {
        Package::<Unknown>::of_types(PackageType::iter())
    }

    fn of_types<I: Iterator<Item = PackageType>>(types: I) -> Vec<Package<Unknown>> {
        types
            .map(|ty| Package::<Unknown> {
                ty,
                meta: ty.metadata(),
                phantom_data: PhantomData {},
            })
            .collect()
    }

    #[instrument(level = "info", fields(package = self.meta.package_name.as_ref()))]
    pub async fn remove(&self) -> Result<()> {
        let download_path = self.download_path();
        if download_path.exists() {
            info!(?download_path, "Removing...");
            tokio::fs::remove_dir_all(&download_path).await?;
        }
        Ok(())
    }

    #[instrument(level = "info")]
    pub fn download(self) -> Result<Package<Downloaded>> {
        let download_path = self.download_path();
        info!(?download_path, "Downloading...");

        match &self.meta.source {
            PackageSource::Git { url, target } => {
                if download_path.exists() {
                    info!(?download_path, "Directory exists. Assuming git repository.");
                    git::checkout(target, &download_path)?;
                } else {
                    info!(
                        ?download_path,
                        "Directory does not exist. Cloning the repository."
                    );
                    git::clone(url, target, &download_path).or_else(|_err| {
                        info!("Direct clone unsuccessful. Trying clone with checkout...");
                        git::clone_without_checkout(url, &download_path)?;
                        git::checkout(target, &download_path)
                    })?;
                }
            }
        };

        Ok(Package::<Downloaded> {
            ty: self.ty,
            meta: self.meta,
            phantom_data: PhantomData {},
        })
    }
}

impl Package<Downloaded> {
    pub fn icons_path(&self) -> PathBuf {
        self.download_path().join(self.meta.svg_dir.as_ref())
    }

    pub async fn read_icons(&self) -> Result<Vec<SvgIcon>> {
        reader::read_icons(self).await
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter, Clone, Copy)]
pub(crate) enum PackageType {
    AntDesignIcons,
    FontAwesome,
    WeatherIcons,
    Feather,
    VSCodeIcons,
    BootstrapIcons,
    BoxIcons,
    IcoMoonFree,
    Ionicons,
    RemixIcon,
    SimpleIcons,
    Typicons,
    Heroicons,
    CssGg,
    TablerIcons,
    GithubOcticons,
}

#[derive(Debug, Clone)]
pub(crate) struct PackageMetadata {
    /// Two-character identifier like "fa" for "Font Awesome".
    pub short_name: Cow<'static, str>,
    /// Full human readable name of this icon package.
    pub package_name: Cow<'static, str>,
    /// Licenses of the icon package.
    pub licenses: &'static [Cow<'static, str>],
    /// The source of this icon package.
    pub source: PackageSource,
    /// Directory to which the source should be downloaded.
    pub download_dir: Cow<'static, str>,
    /// Directory relative to download_dir under which raw SVG files can be found.
    pub svg_dir: Cow<'static, str>, // TODO: PathBuf?
}

#[derive(Debug, Clone)]
pub(crate) enum PackageSource {
    Git {
        url: Cow<'static, str>, // TODO use url type?
        target: GitTarget,
    },
}

#[derive(Debug, Clone)]
pub(crate) enum GitTarget {
    Branch {
        name: Cow<'static, str>,
        commit_ref: Cow<'static, str>,
        version_hint: Option<SemVer>,
    },
    Tag {
        name: Cow<'static, str>,
        version: SemVer,
    },
}

impl PackageType {
    /// Test whether a particular string represents a category of this icon package.
    pub fn is_category(&self, str: &str) -> bool {
        // We avoid using all-numeric directories as categories,
        // as they most likely state the size of the icons contained and not an actual category.
        if str.chars().all(char::is_numeric) {
            return false;
        }
        match self {
            // SVG's located in the "logos" directory are distinct from files in the "regular" and "solid" directories. We may not use that as a category.
            PackageType::BoxIcons => str != "logos",
            _ => true,
        }
    }

    fn metadata(&self) -> PackageMetadata {
        match self {
            PackageType::AntDesignIcons => PackageMetadata {
                short_name: Cow::Borrowed("ai"),
                package_name: Cow::Borrowed("Ant Design Icons"),
                licenses: &[Cow::Borrowed("MIT")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/ant-design/ant-design-icons"),
                    target: GitTarget::Branch {
                        name: Cow::Borrowed("master"),
                        commit_ref: Cow::Borrowed("7c804893b4ac698d5713b2b59f3d044eb8f5128f"),
                        version_hint: Some(SemVer {
                            major: 5,
                            minor: 3,
                            patch: 2,
                            prerelease: None,
                            build: None,
                        }),
                    },
                },
                download_dir: "ant-design-icons".into(),
                svg_dir: Cow::Borrowed("packages/icons-svg/svg"),
            },
            PackageType::FontAwesome => PackageMetadata {
                short_name: Cow::Borrowed("fa"),
                package_name: Cow::Borrowed("Font Awesome"),
                licenses: &[Cow::Borrowed("CC BY 4.0")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/FortAwesome/Font-Awesome"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("6.3.0"),
                        version: SemVer {
                            major: 6,
                            minor: 3,
                            patch: 0,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("font-awesome"),
                svg_dir: Cow::Borrowed("svgs"),
            },
            PackageType::WeatherIcons => PackageMetadata {
                short_name: Cow::Borrowed("wi"),
                package_name: Cow::Borrowed("Weather Icons"),
                licenses: &[Cow::Borrowed("SIL OFL 1.1")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/erikflowers/weather-icons"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("2.0.12"),
                        version: SemVer {
                            major: 2,
                            minor: 0,
                            patch: 12,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("weather-icons"),
                svg_dir: Cow::Borrowed("svg"),
            },
            PackageType::Feather => PackageMetadata {
                short_name: Cow::Borrowed("fi"),
                package_name: Cow::Borrowed("Feather"),
                licenses: &[Cow::Borrowed("MIT")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/feathericons/feather"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("v4.29.0"),
                        version: SemVer {
                            major: 4,
                            minor: 29,
                            patch: 0,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("feather"),
                svg_dir: Cow::Borrowed("icons"),
            },
            PackageType::VSCodeIcons => PackageMetadata {
                short_name: Cow::Borrowed("vs"),
                package_name: Cow::Borrowed("VS Code Icons"),
                licenses: &[Cow::Borrowed("CC BY 4.0")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/microsoft/vscode-codicons"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("0.0.32"),
                        version: SemVer {
                            major: 0,
                            minor: 0,
                            patch: 32,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("vscode-codicons"),
                svg_dir: Cow::Borrowed("src/icons"),
            },
            PackageType::BootstrapIcons => PackageMetadata {
                short_name: Cow::Borrowed("bs"),
                package_name: Cow::Borrowed("Bootstrap Icons"),
                licenses: &[Cow::Borrowed("MIT")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/twbs/icons"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("v1.10.3"),
                        version: SemVer {
                            major: 1,
                            minor: 10,
                            patch: 3,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("bootstrap-icons"),
                svg_dir: Cow::Borrowed("icons"),
            },
            PackageType::BoxIcons => PackageMetadata {
                short_name: Cow::Borrowed("bi"),
                package_name: Cow::Borrowed("BoxIcons"),
                licenses: &[Cow::Borrowed("CC BY 4.0")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/atisawd/boxicons"),
                    target: GitTarget::Branch {
                        name: Cow::Borrowed("master"),
                        commit_ref: Cow::Borrowed("9ffa9136e8681886bb7bd2145cd4098717ce1c11"),
                        version_hint: Some(SemVer {
                            major: 2,
                            minor: 1,
                            patch: 4,
                            prerelease: None,
                            build: None,
                        }),
                    },
                },
                download_dir: Cow::Borrowed("boxicons"),
                svg_dir: Cow::Borrowed("svg"),
            },
            PackageType::IcoMoonFree => PackageMetadata {
                short_name: Cow::Borrowed("im"),
                package_name: Cow::Borrowed("IcoMoon Free"),
                licenses: &[Cow::Borrowed("CC BY 4.0"), Cow::Borrowed("GPL")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/Keyamoon/IcoMoon-Free"),
                    target: GitTarget::Branch {
                        name: Cow::Borrowed("master"),
                        commit_ref: Cow::Borrowed("d006795ede82361e1bac1ee76f215cf1dc51e4ca"),
                        version_hint: None,
                    },
                },
                download_dir: Cow::Borrowed("icomoon-free"),
                svg_dir: Cow::Borrowed("svg"),
            },
            PackageType::Ionicons => PackageMetadata {
                short_name: Cow::Borrowed("io"),
                package_name: Cow::Borrowed("Ionicons"),
                licenses: &[Cow::Borrowed("MIT")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/ionic-team/ionicons"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("v7.1.0"),
                        version: SemVer {
                            major: 7,
                            minor: 1,
                            patch: 0,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("ionicons"),
                svg_dir: Cow::Borrowed("src/svg"),
            },
            PackageType::RemixIcon => PackageMetadata {
                short_name: Cow::Borrowed("ri"),
                package_name: Cow::Borrowed("Remix Icon"),
                licenses: &[Cow::Borrowed("Apache 2.0")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/Remix-Design/RemixIcon"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("v2.5.0"),
                        version: SemVer {
                            major: 2,
                            minor: 5,
                            patch: 0,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("RemixIcon"),
                svg_dir: Cow::Borrowed("icons"),
            },
            PackageType::SimpleIcons => PackageMetadata {
                short_name: Cow::Borrowed("si"),
                package_name: Cow::Borrowed("Simple Icons"),
                licenses: &[Cow::Borrowed("CC0 1.0 Universal")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/simple-icons/simple-icons"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("8.8.0"),
                        version: SemVer {
                            major: 8,
                            minor: 8,
                            patch: 0,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("simple-icons"),
                svg_dir: Cow::Borrowed("icons"),
            },
            PackageType::Typicons => PackageMetadata {
                short_name: Cow::Borrowed("ti"),
                package_name: Cow::Borrowed("Typicons"),
                licenses: &[Cow::Borrowed("CC BY-SA 3.0")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/stephenhutchings/typicons.font"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("v2.1.2"),
                        version: SemVer {
                            major: 2,
                            minor: 1,
                            patch: 2,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("typicons"),
                svg_dir: Cow::Borrowed("src/svg"),
            },
            PackageType::Heroicons => PackageMetadata {
                short_name: Cow::Borrowed("hi"),
                package_name: Cow::Borrowed("Heroicons"),
                licenses: &[Cow::Borrowed("MIT")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/refactoringui/heroicons"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("v2.0.16"),
                        version: SemVer {
                            major: 2,
                            minor: 0,
                            patch: 16,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("heroicons"),
                svg_dir: Cow::Borrowed("src"),
            },
            PackageType::CssGg => PackageMetadata {
                short_name: Cow::Borrowed("cg"),
                package_name: Cow::Borrowed("css.gg"),
                licenses: &[Cow::Borrowed("MIT")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/astrit/css.gg"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("2.0.0"),
                        version: SemVer {
                            major: 2,
                            minor: 0,
                            patch: 0,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("css.gg"),
                svg_dir: Cow::Borrowed("icons/svg"),
            },
            PackageType::TablerIcons => PackageMetadata {
                short_name: Cow::Borrowed("tb"),
                package_name: Cow::Borrowed("Tabler Icons"),
                licenses: &[Cow::Borrowed("MIT")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/tabler/tabler-icons"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("v2.11.0"),
                        version: SemVer {
                            major: 2,
                            minor: 11,
                            patch: 0,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("tabler-icons"),
                svg_dir: Cow::Borrowed("icons"),
            },
            PackageType::GithubOcticons => PackageMetadata {
                short_name: Cow::Borrowed("oc"),
                package_name: Cow::Borrowed("Github Octicons"),
                licenses: &[Cow::Borrowed("MIT")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/primer/octicons"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("v18.3.0"),
                        version: SemVer {
                            major: 18,
                            minor: 3,
                            patch: 0,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("octicons"),
                svg_dir: Cow::Borrowed("icons"),
            },
        }
    }
}
