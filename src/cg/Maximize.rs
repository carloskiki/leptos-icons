#[cfg(feature = "CgMaximize")]
use leptos::*;
#[cfg(feature = "CgMaximize")]
///This icon requires the feature `CgMaximize` to be enabled.
#[component]
pub fn Maximize(
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
        "M3 3H9V5H5V9H3V3Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 21H9V19H5V15H3V21Z" fill = "currentColor"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M15 21H21V15H19V19H15V21Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 3H15V5H19V9H21V3Z" fill = "currentColor" /> < title > { title } < / title >
        < / svg >
    }
}
