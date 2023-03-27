use std::{fmt::Display, str::FromStr};

use anyhow::Result;
use heck::ToPascalCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use xml::attribute::OwnedAttribute;

use crate::{feature::Feature, leptos::LeptosComponent, package::PackageType, svg::Svg};

#[derive(Debug)]
pub(crate) struct SvgIcon {
    pub svg: Svg,
    pub categories: Vec<Category>,
    pub component_name: String,
    pub feature: Feature,
}

impl SvgIcon {
    /// This creates the Rust code for a leptos component representing a single icon.
    /// Feature-gated by the given feature name.
    ///
    /// TODO: Once https://github.com/leptos-rs/leptos/pull/748 is merged, use `::leptos::...` wherever possible and remove `use leptos::*` in main.rs.
    pub(crate) fn create_leptos_icon_component(&self) -> Result<LeptosComponent> {
        let feature_name: &str = &self.feature.name;
        let component_name: &str = &self.component_name;

        let doc_comment = format!("This icon requires the feature `{feature_name}` to be enabled.");
        let component_ident = Ident::new(component_name, Span::call_site());
        let svg_content: TokenStream = self
            .svg
            .content
            .parse()
            .map_err(|_| anyhow::anyhow!("could not transform icon_content to ident"))?;
        let svg_attributes = attributes_token_stream(&self.svg.attributes)?;

        let tokens = quote! {
            #[cfg(feature = #feature_name)]
            #[doc = #doc_comment]
            #[component]
            pub fn #component_ident(
                cx: Scope,
                /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
                #[prop(into, optional, default = String::from("1em"))]
                size: String,
                /// HTML class attribute.
                #[prop(into, optional)]
                class: String,
                /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
                #[prop(into, optional, default = String::from("currentColor"))]
                color: String,
                /// HTML style attribute.
                #[prop(into, optional)]
                style: String,
                /// Accessibility title.
                #[prop(into, optional)]
                title: String,
            ) -> impl IntoView {
                view! { cx,
                    <svg
                        class=class
                        stroke="currentColor"
                        fill="currentColor"
                        stroke_width="0"
                        style=format!("{} color: {};", style, color)
                        #svg_attributes
                        width=size.clone()
                        height=size
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        #svg_content
                        <title>{title}</title>
                    </svg>
                }
            }
        };

        let tokens_file: syn::File = syn::parse2(tokens)?;
        Ok(LeptosComponent(prettyplease::unparse(&tokens_file)))
    }
}

pub(crate) struct IconMeta {
    pub name: String, // Both the component and feature name!
    pub categories: Vec<Category>,
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

fn attributes_token_stream(attributes: &[OwnedAttribute]) -> Result<TokenStream> {
    attributes
        .iter()
        .map(|attribute| {
            let attribute_val = &attribute.value;
            let attr_ident: TokenStream = attribute
                .name
                .local_name
                .parse()
                .map_err(|_| anyhow::anyhow!("could not convert attributes to token stream"))?;
            Ok(quote!(#attr_ident=#attribute_val))
        })
        .collect::<Result<TokenStream>>()
}
