#[cfg(feature = "HiLgOutlineArrowsPointingIn")]
use leptos::*;
#[cfg(feature = "HiLgOutlineArrowsPointingIn")]
///This icon requires the feature `HiLgOutlineArrowsPointingIn` to be enabled.
#[component]
pub fn ArrowsPointingIn(
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
        "M9 9L9 4.5M9 9L4.5 9M9 9L3.75 3.75M9 15L9 19.5M9 15L4.5 15M9 15L3.75 20.25M15 9H19.5M15 9V4.5M15 9L20.25 3.75M15 15H19.5M15 15L15 19.5M15 15L20.25 20.25"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
