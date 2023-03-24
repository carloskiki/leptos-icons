#[cfg(feature = "CgMenuHotdog")]
use leptos::*;
#[cfg(feature = "CgMenuHotdog")]
///This icon requires the feature `CgMenuHotdog` to be enabled.
#[component]
pub fn MenuHotdog(
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
        "M7 6C5.34315 6 4 7.34315 4 9H20C20 7.34315 18.6569 6 17 6H7Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 18C5.34315 18 4 16.6569 4 15H20C20 16.6569 18.6569 18 17 18H7Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 11C2.44772 11 2 11.4477 2 12C2 12.5523 2.44772 13 3 13H21C21.5523 13 22 12.5523 22 12C22 11.4477 21.5523 11 21 11H3Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
