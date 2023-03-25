#[cfg(feature = "CgGhost")]
use leptos::*;
#[cfg(feature = "CgGhost")]
///This icon requires the feature `CgGhost` to be enabled.
#[component]
pub fn Ghost(
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
        "M3 4H15V8H3V4Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 8H17V4H21V8Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M3 10H21V14H3V10Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11 16H3V20H11V16Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 16V20H21V16H13Z" fill = "currentColor" /> <
        title > { title } < / title > < / svg >
    }
}
