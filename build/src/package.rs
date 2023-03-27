use std::borrow::Cow;
use strum::EnumIter;

use crate::sem_ver::SemVer;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter, Clone, Copy)]
pub(crate) enum Package {
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

impl Package {
    pub fn is_category(&self, str: &str) -> bool {
        // We avoid using all-numeric directories as categories,
        // as they most likely state the size of the icons contained and not an actual category.
        if str.chars().all(char::is_numeric) {
            return false;
        }
        match self {
            Package::AntDesignIcons => true,
            Package::FontAwesome => true,
            Package::WeatherIcons => true,
            Package::Feather => true,
            Package::VSCodeIcons => true,
            Package::BootstrapIcons => true,
            // SVG's located in the "logos" directory are distinct from files in the "regular" and "solid" directories. We may not use that as a category.
            Package::BoxIcons => str != "logos",
            Package::IcoMoonFree => true,
            Package::Ionicons => true,
            Package::RemixIcon => true,
            Package::SimpleIcons => true,
            Package::Typicons => true,
            Package::Heroicons => true,
            Package::CssGg => true,
            Package::TablerIcons => true,
            Package::GithubOcticons => true,
        }
    }
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

#[derive(Debug, Clone)]
pub(crate) struct PackageMetadata {
    /// Two-character identifier like "fa" for "Font Awesome".
    pub short_name: Cow<'static, str>,
    /// Full human readable name of this icon package.
    pub package_name: Cow<'static, str>,
    /// Licenses of the icon package.
    pub license: &'static [Cow<'static, str>],
    /// The source of this icon package.
    pub source: PackageSource,
    /// Directory to which the source should be downloaded.
    pub download_dir: Cow<'static, str>,
    /// Directory relative to download_dir under which raw SVG files can be found.
    pub svg_dir: Cow<'static, str>, // TODO: PathBuf?
}

impl Package {
    pub fn metadata(&self) -> PackageMetadata {
        match self {
            Package::AntDesignIcons => PackageMetadata {
                short_name: Cow::Borrowed("ai"),
                package_name: Cow::Borrowed("Ant Design Icons"),
                license: &[Cow::Borrowed("MIT")],
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
            Package::FontAwesome => PackageMetadata {
                short_name: Cow::Borrowed("fa"),
                package_name: Cow::Borrowed("Font Awesome"),
                license: &[Cow::Borrowed("CC BY 4.0 License")],
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
            Package::WeatherIcons => PackageMetadata {
                short_name: Cow::Borrowed("wi"),
                package_name: Cow::Borrowed("Weather Icons"),
                license: &[Cow::Borrowed("SIL OFL 1.1")],
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
            Package::Feather => PackageMetadata {
                short_name: Cow::Borrowed("fi"),
                package_name: Cow::Borrowed("Feather"),
                license: &[Cow::Borrowed("MIT")],
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
            Package::VSCodeIcons => PackageMetadata {
                short_name: Cow::Borrowed("vs"),
                package_name: Cow::Borrowed("VS Code Icons"),
                license: &[Cow::Borrowed("CC BY 4.0")],
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
            Package::BootstrapIcons => PackageMetadata {
                short_name: Cow::Borrowed("bs"),
                package_name: Cow::Borrowed("Bootstrap Icons"),
                license: &[Cow::Borrowed("MIT")],
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
            Package::BoxIcons => PackageMetadata {
                short_name: Cow::Borrowed("bi"),
                package_name: Cow::Borrowed("BoxIcons"),
                license: &[Cow::Borrowed("CC BY 4.0 License")],
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
            Package::IcoMoonFree => PackageMetadata {
                short_name: Cow::Borrowed("im"),
                package_name: Cow::Borrowed("IcoMoon Free"),
                license: &[Cow::Borrowed("CC BY 4.0 License"), Cow::Borrowed("GPL")],
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
            Package::Ionicons => PackageMetadata {
                short_name: Cow::Borrowed("io"),
                package_name: Cow::Borrowed("Ionicons"),
                license: &[Cow::Borrowed("MIT")],
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
            Package::RemixIcon => PackageMetadata {
                short_name: Cow::Borrowed("ri"),
                package_name: Cow::Borrowed("Remix Icon"),
                license: &[Cow::Borrowed("Apache License Version 2.0")],
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
            Package::SimpleIcons => PackageMetadata {
                short_name: Cow::Borrowed("si"),
                package_name: Cow::Borrowed("Simple Icons"),
                license: &[Cow::Borrowed("CC0 1.0 Universal")],
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
            Package::Typicons => PackageMetadata {
                short_name: Cow::Borrowed("ti"),
                package_name: Cow::Borrowed("Typicons"),
                license: &[Cow::Borrowed("CC BY-SA 3.0")],
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
            Package::Heroicons => PackageMetadata {
                short_name: Cow::Borrowed("hi"),
                package_name: Cow::Borrowed("Heroicons"),
                license: &[Cow::Borrowed("MIT")],
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
            Package::CssGg => PackageMetadata {
                short_name: Cow::Borrowed("cg"),
                package_name: Cow::Borrowed("css.gg"),
                license: &[Cow::Borrowed("MIT")],
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
            Package::TablerIcons => PackageMetadata {
                short_name: Cow::Borrowed("tb"),
                package_name: Cow::Borrowed("Tabler Icons"),
                license: &[Cow::Borrowed("MIT")],
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
            Package::GithubOcticons => PackageMetadata {
                short_name: Cow::Borrowed("oc"),
                package_name: Cow::Borrowed("Github Octicons"),
                license: &[Cow::Borrowed("MIT")],
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
