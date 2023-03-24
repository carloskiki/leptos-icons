#[cfg(feature = "CgSidebar")]
use leptos::*;
#[cfg(feature = "CgSidebar")]
///This icon requires the feature `CgSidebar` to be enabled.
#[component]
pub fn Sidebar(
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
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M21 20H7V4H21V20ZM19 18H9V6H19V18Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 20H5V4H3V20Z" fill = "currentColor" /> <
        title > { title } < / title > < / svg >
    }
}
