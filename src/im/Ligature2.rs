#[cfg(feature = "ImLigature2")]
use leptos::*;
#[cfg(feature = "ImLigature2")]
///This icon requires the feature `ImLigature2` to be enabled.
#[component]
pub fn Ligature2(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    let style = format!("{} color: {};", style, color);
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M13.364 14.335c-0.183 0-1.307-0.206-1.375-0.458-0.161-0.619-0.183-1.284-0.183-2.040v-8.453c0-1.261 0.252-1.994 0.252-1.994-0.023-0.115-0.138-0.367-0.275-0.367h-0.069c-0.069 0-0.871 0.504-1.605 0.504-0.596-0-0.967-0.527-1.655-0.527-2.892 0-4.249 2.349-4.249 5.672v0.173c0 0.069-0.046 0.138-0.115 0.138h-0.94c-0.115 0-0.344 0.642-0.344 0.94 0 0.092 0.023 0.137 0.069 0.137h1.215c0.069 0 0.115 0.092 0.115 0.16 0 2.040-0.023 4.052-0.023 4.052 0 0.321-0.023 1.031-0.16 1.605-0.069 0.252-1.123 0.458-1.398 0.458-0.115 0-0.115 0.55 0 0.665 0.94-0.046 1.559-0.115 2.499-0.115 0.871 0 1.536 0.069 2.453 0.115 0.046-0.138 0.046-0.665-0.069-0.665-0.183 0-1.307-0.206-1.375-0.458-0.16-0.619-0.16-1.284-0.183-2.040v-3.639c0-0.069 0.069-0.138 0.138-0.138h2.361c0.16-0.321 0.275-0.711 0.275-0.917 0-0.138 0-0.16-0.115-0.16h-2.544c-0.046 0-0.115-0.069-0.115-0.115v-0.825c0-2.040 0.836-3.837 2.234-3.837 0.99 0 1.854 0.642 1.854 3.093 0 0 0 0 0 0 0.003 0.063 0.005 0.114 0.005 0.148v6.825c0 0.321-0.023 1.031-0.16 1.605-0.069 0.252-1.123 0.458-1.398 0.458-0.115 0-0.115 0.55 0 0.665 0.94-0.046 1.559-0.115 2.499-0.115 0.871 0 1.536 0.069 2.453 0.115 0.046-0.137 0.046-0.665-0.069-0.665z"
        /> < title > { title } < / title > < / svg >
    }
}
