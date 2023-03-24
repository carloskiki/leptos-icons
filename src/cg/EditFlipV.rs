#[cfg(feature = "CgEditFlipV")]
use leptos::*;
#[cfg(feature = "CgEditFlipV")]
///This icon requires the feature `CgEditFlipV` to be enabled.
#[component]
pub fn EditFlipV(
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
        "M17 18C17 18.5523 16.5523 19 16 19L8 19C7.44772 19 7 18.5523 7 18L7 15L5 15L5 18C5 19.6569 6.34315 21 8 21L16 21C17.6569 21 19 19.6569 19 18V15L17 15V18Z"
        fill = "currentColor" fill - opacity = "0.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M16 5C16.5523 5 17 5.44772 17 6V9H19V6C19 4.34315 17.6569 3 16 3L8 3C6.34315 3 5 4.34315 5 6V9H7V6C7 5.44772 7.44772 5 8 5L16 5Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 13V11L3 11V13H21Z" fill = "currentColor" /> < title > { title } < / title >
        < / svg >
    }
}
