use std::{fmt::Display, str::FromStr};

use heck::ToPascalCase;
use tracing::info;

use crate::package::Package;

#[derive(Debug)]
pub(crate) struct Icon {
    pub view: String,
    pub size: Option<IconSize>,
    pub categories: Vec<String>,
    pub component_name: String,
    pub feature_name: String,
    // TODO: Original file name?
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) enum IconSize {
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

impl IconSize {
    fn as_str(&self) -> &'static str {
        match self {
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
pub(crate) fn component_name(raw_name: &str) -> String {
    let pascal_case = raw_name.to_pascal_case();
    if pascal_case.starts_with(char::is_numeric) {
        format!("_{pascal_case}")
    } else {
        pascal_case
    }
}

#[cfg(test)]
mod test {
    use crate::icon::component_name;

    #[test]
    fn test() {
        assert_eq!("TestName", component_name("1-test-name"))
    }
}
pub(crate) fn feature_name(
    raw_name: &str,
    icon_size: Option<IconSize>,
    categories: &Vec<String>,
    package_short_name: &str,
) -> String {
    let mut name = String::with_capacity(
        package_short_name.len()
            + 1
            + raw_name.len()
            + categories.iter().map(|it| it.len() + 1).sum::<usize>()
            + icon_size.map(|it| it.as_str().len() + 1).unwrap_or(0),
    );

    name.push_str(package_short_name.as_ref());
    name.push(' ');

    name.push_str(raw_name);
    name.push(' ');

    categories.iter().for_each(|category| {
        name.push_str(&category);
        name.push(' ');
    });

    if let Some(size) = icon_size {
        name.push_str(size.as_str());
        name.push(' ');
    };

    name.to_pascal_case()
}

pub(crate) fn extract_raw_icon_name(
    package: Package,
    file_stem: &str,
) -> (&str, Option<IconSize>, Option<Vec<String>>) {
    match package {
        // octoicons: size suffix e.g: '-24.svg'
        Package::GithubOcticons => {
            let size = IconSize::from_str(&file_stem[(file_stem.len() - 2)..]).ok();
            let name = file_stem
                .trim_end_matches(char::is_numeric)
                .trim_end_matches('-');
            (name, size, None)
        }
        // Weather icons: 'wi-' prefix
        Package::WeatherIcons => {
            let name = file_stem.trim_start_matches("wi-");
            (name, None, None)
        }
        // Box icons: logos: 'bxl-', regular:  'bx-', solid: 'bxs-' prefixes
        Package::BoxIcons => {
            let name = file_stem
                .trim_start_matches("bxl-")
                .trim_start_matches("bx-")
                .trim_start_matches("bxs-");
            (name, None, None)
        }
        // Icomoon icons: numbered '001-xxxxxx'
        Package::IcoMoonFree => {
            let name = file_stem.trim_start_matches(char::is_numeric);
            (name, None, None)
        }
        Package::RemixIcon => {
            let mut name = file_stem;
            let mut cats = vec![];
            if name.ends_with("-fill") {
                name = name.trim_end_matches("-fill");
                cats.push("fill".to_string());
            } else if name.ends_with("-line") {
                name = name.trim_end_matches("-line");
                cats.push("line".to_string());
            }
            (name, None, if cats.is_empty() { None } else { Some(cats) })
        }
        _ => (file_stem, None, None),
    }
}

pub(crate) fn gen_icon_components(package: Package, icons: Vec<Icon>) -> Vec<LeptosComponent> {
    info!(?package, "Generating leptos icon components.");
    icons
        .into_iter()
        .map(|icon| {
            create_leptos_icon_component(&icon.feature_name, &icon.component_name, &icon.view)
        })
        .collect()
}

/// A self-contained Leptos component declaration.
///
/// TODO: Once https://github.com/leptos-rs/leptos/pull/748 is merged, remove this note
/// Currently requires `use leptos::*;` to be in scope to compile properly.
pub(crate) struct LeptosComponent(pub String);

/// This creates the Rust code for a leptos component representing a single icon.
/// Feature-gated by the given feature name.
///
/// TODO: Once https://github.com/leptos-rs/leptos/pull/748 is merged, use `::leptos::...` wherever possible and remove `use leptos::*` in main.rs.
fn create_leptos_icon_component(
    feature_name: &str,
    component_name: &str,
    view: &str,
) -> LeptosComponent {
    LeptosComponent(indoc::formatdoc!(
        r#"
        #[cfg(feature = "{feature_name}")]
        /// *This icon requires the feature* `{feature_name}` *to be enabled*.
        #[component]
        pub fn {component_name}(cx: Scope) -> impl IntoView {{
            view! {{ cx,
                {view}
            }}
        }}

        "#
    ))
}
