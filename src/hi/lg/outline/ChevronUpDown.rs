#[cfg(feature = "HiLgOutlineChevronUpDown")]
use leptos::*;
#[cfg(feature = "HiLgOutlineChevronUpDown")]
///This icon requires the feature `HiLgOutlineChevronUpDown` to be enabled.
#[component]
pub fn ChevronUpDown(
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
        "http://www.w3.org/2000/svg" d =
        "M8.25 15L12 18.75L15.75 15M8.25 9L12 5.25L15.75 9" stroke = "#0F172A" stroke -
        width = "1.5" stroke - linecap = "round" stroke - linejoin = "round" /> < title >
        { title } < / title > < / svg >
    }
}
