use std::{fmt::Display, str::FromStr};

use heck::ToPascalCase;

use crate::package::Package;

#[derive(Debug)]
pub(crate) struct Icon {
    pub(crate) content: String,
    pub(crate) name: IconName,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) enum IconSize {
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

impl Display for IconSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            IconSize::Sm => "sm",
            IconSize::Md => "md",
            IconSize::Lg => "lg",
            IconSize::Xl => "xl",
            IconSize::Xxl => "xxl",
        })
    }
}

impl FromStr for IconSize {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "16" => Ok(IconSize::Sm),
            "20" => Ok(IconSize::Md),
            "24" => Ok(IconSize::Lg),
            "48" => Ok(IconSize::Xl),
            "96" => Ok(IconSize::Xxl),
            other => {
                return Err(anyhow::anyhow!(
                    "Icon size '{other}' could not be recognized!"
                ))
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct IconName {
    pub(crate) size: Option<IconSize>,
    pub(crate) categories: Vec<Category>,
    pub(crate) name: String,
}

// Ant design icons: filled, outlined, twotone
// Font awesome: brands, regular, solid
// Feather icons: Not categorized
// VS code icons: light, dark
// Bootstrap icons: Not categorized
// Ionicons: Not categorized
// Remix icons: tons of categories
// Simple icons: Not categorized
// Typicons: Not categorized
// Heroicons: (20| solid) , (24| outline, solid)
// css.gg: Not categorized
// Tabler icons: Not categorized
impl IconName {
    pub(crate) fn component_name(&self) -> String {
        let name = self.name.to_pascal_case();
        if name.starts_with(char::is_numeric) {
            return "_".to_owned() + &name;
        }
        name
    }

    pub(crate) fn feature_name(&self, package_short_name: &str) -> String {
        let mut name = package_short_name.to_owned() + " ";

        if let Some(size) = self.size {
            name = name + &size.to_string() + " ";
        };
        self.categories.iter().for_each(|category| {
            name.push_str(category.to_str());
            name.push(' ');
        });
        name.push_str(&self.name);

        name.to_pascal_case()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Category {
    Fill,
    Line,
    Other(String),
}

impl Category {
    fn to_str(&self) -> &str {
        match self {
            Category::Fill => "fill",
            Category::Line => "line",
            Category::Other(str) => str.as_str(),
        }
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

impl IconName {
    pub(crate) fn new(
        package: Package,
        mut name: &str,
        mut size: Option<IconSize>,
        mut categories: Vec<Category>,
    ) -> IconName {
        match package {
            // octoicons: size suffix e.g: '-24.svg'
            Package::GithubOcticons => {
                size = IconSize::from_str(&name[(name.len() - 2)..]).ok();
                name = name
                    .trim_end_matches(char::is_numeric)
                    .trim_end_matches('-');
            }
            // Weather icons: 'wi-' prefix
            Package::WeatherIcons => {
                name = name.trim_start_matches("wi-");
            }
            // Box icons: logos: 'bxl-', regular:  'bx-', solid: 'bxs-' prefixes
            Package::BoxIcons => {
                name = name
                    .trim_start_matches("bxl-")
                    .trim_start_matches("bx-")
                    .trim_start_matches("bxs-");
            }
            // Icomoon icons: numbered '001-xxxxxx'
            Package::IcoMoonFree => {
                name = name.trim_start_matches(char::is_numeric);
            }
            Package::RemixIcon => {
                if name.ends_with("-fill") {
                    name = name.trim_end_matches("-fill");
                    categories.push(Category::Fill);
                } else if name.ends_with("-line") {
                    name = name.trim_end_matches("-line");
                    categories.push(Category::Line);
                }
            }
            _ => (),
        };

        let name = name.to_string();

        IconName {
            size,
            categories,
            name,
        }
    }
}
