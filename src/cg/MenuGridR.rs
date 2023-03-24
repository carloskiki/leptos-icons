#[cfg(feature = "CgMenuGridR")]
use leptos::*;
#[cfg(feature = "CgMenuGridR")]
///This icon requires the feature `CgMenuGridR` to be enabled.
#[component]
pub fn MenuGridR(
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
        "M4 4H8V8H4V4Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 10H8V14H4V10Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M8 16H4V20H8V16Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 4H14V8H10V4Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 10H10V14H14V10Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M10 16H14V20H10V16Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 4H16V8H20V4Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 10H20V14H16V10Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M20 16H16V20H20V16Z" fill =
        "currentColor" /> < title > { title } < / title > < / svg >
    }
}
