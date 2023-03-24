#[cfg(feature = "CgFramer")]
use leptos::*;
#[cfg(feature = "CgFramer")]
///This icon requires the feature `CgFramer` to be enabled.
#[component]
pub fn Framer(
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
        "http://www.w3.org/2000/svg" d = "M12 21L12 9L6 9L6 15L12 21Z" fill =
        "currentColor" fill - opacity = "0.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 9V3H6L12 9H6V15H18L12 9H18Z" fill =
        "currentColor" /> < title > { title } < / title > < / svg >
    }
}
