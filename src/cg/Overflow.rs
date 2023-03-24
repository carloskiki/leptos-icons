#[cfg(feature = "CgOverflow")]
use leptos::*;
#[cfg(feature = "CgOverflow")]
///This icon requires the feature `CgOverflow` to be enabled.
#[component]
pub fn Overflow(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg"
        opacity = "0.2" d =
        "M22 11C22 13.6522 20.9464 16.1957 19.0711 18.0711C17.1957 19.9464 14.6522 21 12 21C9.34784 21 6.8043 19.9464 4.92893 18.0711C3.05357 16.1957 2 13.6522 2 11L22 11Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" opacity =
        "0.3" d =
        "M20 11C20 13.1217 19.1571 15.1566 17.6569 16.6569C16.1566 18.1571 14.1217 19 12 19C9.87827 19 7.84344 18.1571 6.34315 16.6569C4.84286 15.1566 4 13.1217 4 11L20 11Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 11C20 8.87827 19.1571 6.84344 17.6569 5.34315C16.1566 3.84285 14.1217 3 12 3C9.87827 3 7.84344 3.84285 6.34315 5.34315C4.84286 6.84344 4 8.87827 4 11L20 11Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
