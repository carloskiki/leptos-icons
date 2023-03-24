#[cfg(feature = "ImManWoman")]
use leptos::*;
#[cfg(feature = "ImManWoman")]
///This icon requires the feature `ImManWoman` to be enabled.
#[component]
pub fn ManWoman(
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
        "M4 1.5c0 0.828-0.672 1.5-1.5 1.5s-1.5-0.672-1.5-1.5c0-0.828 0.672-1.5 1.5-1.5s1.5 0.672 1.5 1.5z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M13 1.5c0 0.828-0.672 1.5-1.5 1.5s-1.5-0.672-1.5-1.5c0-0.828 0.672-1.5 1.5-1.5s1.5 0.672 1.5 1.5z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M4 4h-3c-0.552 0-1 0.448-1 1v5h1v6h1.25v-6h0.5v6h1.25v-6h1v-5c0-0.552-0.448-1-1-1z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M15.234 8l0.766-0.555-2.083-3.221c-0.092-0.14-0.249-0.225-0.417-0.225h-4c-0.168 0-0.325 0.084-0.417 0.225l-2.083 3.221 0.766 0.555 1.729-2.244 0.601 1.402-2.095 3.841h1.917l0.333 5h1v-5h0.5v5h1l0.333-5h1.917l-2.095-3.842 0.601-1.402 1.729 2.244z"
        /> < title > { title } < / title > < / svg >
    }
}
