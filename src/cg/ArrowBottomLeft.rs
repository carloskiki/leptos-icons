#[cfg(feature = "CgArrowBottomLeft")]
use leptos::*;
#[cfg(feature = "CgArrowBottomLeft")]
///This icon requires the feature `CgArrowBottomLeft` to be enabled.
#[component]
pub fn ArrowBottomLeft(
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
        "M18.2426 7.17154L16.8284 5.75732L7.75739 14.8283L7.75739 10.2427H5.75739L5.75739 18.2427H13.7574V16.2427L9.17144 16.2427L18.2426 7.17154Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
