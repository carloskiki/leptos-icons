#[cfg(feature = "HiLgOutlineArrowRightOnRectangle")]
use leptos::*;
#[cfg(feature = "HiLgOutlineArrowRightOnRectangle")]
///This icon requires the feature `HiLgOutlineArrowRightOnRectangle` to be enabled.
#[component]
pub fn ArrowRightOnRectangle(
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
        "M15.75 9V5.25C15.75 4.00736 14.7426 3 13.5 3L7.5 3C6.25736 3 5.25 4.00736 5.25 5.25L5.25 18.75C5.25 19.9926 6.25736 21 7.5 21H13.5C14.7426 21 15.75 19.9926 15.75 18.75V15M18.75 15L21.75 12M21.75 12L18.75 9M21.75 12L9 12"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
