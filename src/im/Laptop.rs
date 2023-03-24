#[cfg(feature = "ImLaptop")]
use leptos::*;
#[cfg(feature = "ImLaptop")]
///This icon requires the feature `ImLaptop` to be enabled.
#[component]
pub fn Laptop(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M14 11v-8c0-0.55-0.45-1-1-1h-10c-0.55 0-1 0.45-1 1v8h-2v3h16v-3h-2zM10 13h-4v-1h4v1zM13 11h-10v-7.998c0.001-0.001 0.001-0.001 0.002-0.002h9.996c0.001 0.001 0.001 0.001 0.002 0.002v7.998z"
        /> < title > { title } < / title > < / svg >
    }
}
