#[cfg(feature = "HiLgOutlineArrowLongLeft")]
use leptos::*;
#[cfg(feature = "HiLgOutlineArrowLongLeft")]
///This icon requires the feature `HiLgOutlineArrowLongLeft` to be enabled.
#[component]
pub fn ArrowLongLeft(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d = "M6.75 15.75L3 12M3 12L6.75 8.25M3 12H21" stroke
        = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke - linejoin =
        "round" /> < title > { title } < / title > < / svg >
    }
}
