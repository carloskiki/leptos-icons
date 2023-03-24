#[cfg(feature = "ImCalculator")]
use leptos::*;
#[cfg(feature = "ImCalculator")]
///This icon requires the feature `ImCalculator` to be enabled.
#[component]
pub fn Calculator(
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
        "M6 1h-5c-0.55 0-1 0.45-1 1v5c0 0.55 0.45 1 1 1h5c0.55 0 1-0.45 1-1v-5c0-0.55-0.45-1-1-1zM6 5h-5v-1h5v1zM14 1h-5c-0.55 0-1 0.45-1 1v13c0 0.55 0.45 1 1 1h5c0.55 0 1-0.45 1-1v-13c0-0.55-0.45-1-1-1zM14 10h-5v-1h5v1zM14 7h-5v-1h5v1zM6 9h-5c-0.55 0-1 0.45-1 1v5c0 0.55 0.45 1 1 1h5c0.55 0 1-0.45 1-1v-5c0-0.55-0.45-1-1-1zM6 13h-2v2h-1v-2h-2v-1h2v-2h1v2h2v1z"
        /> < title > { title } < / title > < / svg >
    }
}
