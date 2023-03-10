use std::{path::PathBuf, str::FromStr};
use convert_case::{Casing, Case};
use serde::Deserialize;
use url::Url;
use anyhow::anyhow;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct IconPackage {
    pub(crate) short_name: String,
    pub(crate) package_name: String,
    pub(crate) url: Url,
    pub(crate) path: PathBuf,
    pub(crate) folder_name: String,
    #[serde(skip)]
    pub(crate) icons: Vec<Icon>,
}

#[derive(Debug)]
pub(crate) struct Icon {
    pub(crate) content: String,
    pub(crate) name: IconName,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum IconSize {
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

impl ToString for IconSize {
    fn to_string(&self) -> String {
        match self {
            IconSize::Sm => "sm".to_string(),
            IconSize::Md => "md".to_string(),
            IconSize::Lg => "lg".to_string(),
            IconSize::Xl => "xl".to_string(),
            IconSize::Xxl => "xxl".to_string(),
        }
    }
}

impl FromStr for IconSize {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "16" => Ok(IconSize::Sm),
            "20" => Ok(IconSize::Md),
            "24" => Ok(IconSize::Lg),
            "48" => Ok(IconSize::Xl),
            "96" => Ok(IconSize::Xxl),
            _ => return Err(anyhow!("icon size not recognized")),
        }
    }
}

#[derive(Debug)]
pub(crate) struct IconName {
    pub(crate) size: Option<IconSize>,
    pub(crate) categories: Vec<String>,
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
    pub(crate) fn new(
        mut name: &str,
        package_short_name: &str,
        mut size: Option<IconSize>,
        mut categories: Vec<String>,
    ) -> IconName {
        match package_short_name {
            // octoicons: size suffix e.g: '-24.svg'
            "oc" => {
                size = IconSize::from_str(&name[(name.len() - 2)..]).ok();
                name = name
                    .trim_end_matches(char::is_numeric)
                    .trim_end_matches('-');
            }
            // Weather icons: 'wi-' prefix
            "wi" => {
                name = name.trim_start_matches("wi-");
            }
            // Box icons: logos: 'bxl-', regular:  'bx-', solid: 'bxs-' prefixes
            "bi" => {
                name = name
                    .trim_start_matches("bxl-")
                    .trim_start_matches("bx-")
                    .trim_start_matches("bxs-");
            }
            // Icomoon icons: numbered '001-xxxxxx'
            "im" => {
                name = name.trim_start_matches(char::is_numeric);
            }
            "ri" => {
                if name.ends_with("-fill") {
                    name = name.trim_end_matches("-fill");
                    categories.push("fill".to_string());
                } else if name.ends_with("-line") {
                    name = name.trim_end_matches("-line");
                    categories.push("line".to_string());
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

    pub(crate) fn component_name(&self) -> String {
        self.name.to_case(Case::Pascal)
    }

    pub(crate) fn feature_name(&self, package_short_name: &str) -> String {
        let mut name = package_short_name.to_owned() + " ";

        if let Some(size) = self.size {
            name = name + &size.to_string() + " ";
        };
        self.categories.iter().for_each(|category| {
            name.push_str(category);
            name.push(' ');
        });
        name.push_str(&self.name);

        name.to_case(Case::Pascal)
    }
}
