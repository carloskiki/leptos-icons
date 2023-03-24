#[cfg(feature = "CgMenuOreos")]
use leptos::*;
#[cfg(feature = "CgMenuOreos")]
///This icon requires the feature `CgMenuOreos` to be enabled.
#[component]
pub fn MenuOreos(
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
        "M7 3C5.34315 3 4 4.34315 4 6H20C20 4.34315 18.6569 3 17 3H7Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 11C5.34315 11 4 9.65685 4 8H20C20 9.65685 18.6569 11 17 11H7Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 13C5.34315 13 4 14.3431 4 16H20C20 14.3431 18.6569 13 17 13H7Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 21C5.34315 21 4 19.6569 4 18H20C20 19.6569 18.6569 21 17 21H7Z" fill =
        "currentColor" /> < title > { title } < / title > < / svg >
    }
}
