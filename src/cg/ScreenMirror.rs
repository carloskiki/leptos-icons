#[cfg(feature = "CgScreenMirror")]
use leptos::*;
#[cfg(feature = "CgScreenMirror")]
///This icon requires the feature `CgScreenMirror` to be enabled.
#[component]
pub fn ScreenMirror(
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
        "http://www.w3.org/2000/svg" d = "M5 8H19V14H16V16H21V6H3V16H8V14H5V8Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.3301 19L12 13L7.66987 19H16.3301Z" fill = "currentColor" /> < title > {
        title } < / title > < / svg >
    }
}
