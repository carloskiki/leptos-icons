use std::{fmt::Display, path::Path, str::FromStr};

use anyhow::Result;
use heck::ToPascalCase;

use crate::{
    feature::Feature,
    package::{Downloaded, Package, PackageType},
};

use self::svg::ParsedSvg;

mod svg;

#[derive(Debug, Clone)]
pub(crate) struct SvgIcon {
    pub svg: svg::ParsedSvg,
    pub categories: Vec<Category>,
    pub feature: Feature,
}

impl SvgIcon {
    pub async fn new(
        package: &Package<Downloaded>,
        path: &Path,
        size: Option<IconSize>,
        mut categories: Vec<Category>,
    ) -> Result<Self> {
        let file_stem = path.file_stem().unwrap().to_string_lossy(); // TODO: Error handling\

        let (raw_name, size_from_name, cats_from_name) =
            parse_raw_icon_name(package.ty, &file_stem);

        if let Some(mut cats_from_name) = cats_from_name {
            categories.append(&mut cats_from_name);
        }

        let feature = Feature {
            name: feature_name(
                raw_name,
                size_from_name.or(size),
                &categories,
                &package.meta.short_name,
            ),
        };

        let svg = tokio::fs::read_to_string(path).await?;

        Ok(SvgIcon {
            svg: ParsedSvg::parse(svg.as_bytes())?,
            categories,
            feature,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) struct Category(pub String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) enum IconSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

impl IconSize {
    fn as_str(&self) -> &'static str {
        match self {
            IconSize::Xs => "xs",
            IconSize::Sm => "sm",
            IconSize::Md => "md",
            IconSize::Lg => "lg",
            IconSize::Xl => "xl",
            IconSize::Xxl => "xxl",
        }
    }
}

impl Display for IconSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for IconSize {
    type Err = anyhow::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "12" => Ok(IconSize::Xs),
            "16" => Ok(IconSize::Sm),
            "20" => Ok(IconSize::Md),
            "24" => Ok(IconSize::Lg),
            "48" => Ok(IconSize::Xl),
            "96" => Ok(IconSize::Xxl),
            other => Err(anyhow::anyhow!(
                "Icon size '{other}' could not be recognized!"
            )),
        }
    }
}

pub(crate) fn feature_name(
    raw_name: &str,
    size: Option<IconSize>,
    categories: &[Category],
    package_short_name: &str,
) -> String {
    let mut name = String::with_capacity(
        package_short_name.len()
            + 1
            + raw_name.len()
            + categories.iter().map(|it| it.0.len() + 1).sum::<usize>()
            + size.map(|it| it.as_str().len() + 1).unwrap_or(0),
    );

    name.push_str(package_short_name.as_ref());
    name.push(' ');

    name.push_str(raw_name);
    name.push(' ');

    categories.iter().for_each(|category| {
        name.push_str(&category.0);
        name.push(' ');
    });

    if let Some(size) = size {
        name.push_str(size.as_str());
        name.push(' ');
    };

    name.to_pascal_case()
}

pub(crate) fn parse_raw_icon_name(
    package: PackageType,
    file_stem: &str,
) -> (&str, Option<IconSize>, Option<Vec<Category>>) {
    match package {
        // octoicons: size suffix e.g: '-24.svg'
        PackageType::GithubOcticons => {
            let size = IconSize::from_str(&file_stem[(file_stem.len() - 2)..]).ok();
            let name = file_stem
                .trim_end_matches(char::is_numeric)
                .trim_end_matches('-');
            (name, size, None)
        }
        // Weather icons: 'wi-' prefix
        PackageType::WeatherIcons => {
            let name = file_stem.trim_start_matches("wi-");
            (name, None, None)
        }
        // Box icons: logos: 'bxl-', regular:  'bx-', solid: 'bxs-' prefixes
        PackageType::BoxIcons => {
            let name = file_stem
                .trim_start_matches("bxl-")
                .trim_start_matches("bx-")
                .trim_start_matches("bxs-");
            (name, None, None)
        }
        // Icomoon icons: numbered '001-xxxxxx'
        PackageType::IcoMoonFree => {
            let name = file_stem.trim_start_matches(char::is_numeric);
            (name, None, None)
        }
        PackageType::RemixIcon => {
            let mut name = file_stem;
            let mut cats = vec![];
            if name.ends_with("-fill") {
                name = name.trim_end_matches("-fill");
                cats.push(Category("fill".to_string()));
            } else if name.ends_with("-line") {
                name = name.trim_end_matches("-line");
                cats.push(Category("line".to_string()));
            }
            (name, None, if cats.is_empty() { None } else { Some(cats) })
        }
        _ => (file_stem, None, None),
    }
}
