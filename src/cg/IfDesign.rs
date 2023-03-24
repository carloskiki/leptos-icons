#[cfg(feature = "CgIfDesign")]
use leptos::*;
#[cfg(feature = "CgIfDesign")]
///This icon requires the feature `CgIfDesign` to be enabled.
#[component]
pub fn IfDesign(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 5H14V19H10V5Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 19V10H9V19H5Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 5C5.89543 5 5 5.89543 5 7C5 8.10457 5.89543 9 7 9C8.10457 9 9 8.10457 9 7C9 5.89543 8.10457 5 7 5Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 5H19V9H15V5Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 10H15V14H19V10Z" fill = "currentColor" /> <
        title > { title } < / title > < / svg >
    }
}
