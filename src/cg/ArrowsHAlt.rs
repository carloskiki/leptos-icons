#[cfg(feature = "CgArrowsHAlt")]
use leptos::*;
#[cfg(feature = "CgArrowsHAlt")]
///This icon requires the feature `CgArrowsHAlt` to be enabled.
#[component]
pub fn ArrowsHAlt(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M4.24267 7.75735L5.65688 9.17157L3.82842 11H20.1716L18.3431 9.17157L19.7573 7.75735L24 12L19.7572 16.2426L18.343 14.8284L20.1714 13H3.82845L5.65685 14.8284L4.24264 16.2426L0 12L4.24267 7.75735Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
