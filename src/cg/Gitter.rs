#[cfg(feature = "CgGitter")]
use leptos::*;
#[cfg(feature = "CgGitter")]
///This icon requires the feature `CgGitter` to be enabled.
#[component]
pub fn Gitter(
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
        "http://www.w3.org/2000/svg" d = "M5 1.5H7V14.5H5V1.5Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M9 4.5H11V22.5H9V4.5Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 4.5H13V22.5H15V4.5Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 4.5H19V14.5H17V4.5Z" fill = "currentColor"
        /> < title > { title } < / title > < / svg >
    }
}
