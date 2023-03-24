#[cfg(feature = "CgSketch")]
use leptos::*;
#[cfg(feature = "CgSketch")]
///This icon requires the feature `CgSketch` to be enabled.
#[component]
pub fn Sketch(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M5.20879 3H18.903L20.1118 10.2527L12.0559 21.1858L4 10.2527L5.20879 3ZM6.90304 5L6.11184 9.74726L12.0559 17.8142L18 9.74726L17.2088 5H6.90304Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.05592 8H16.0559V10H8.05592V8Z" fill = "currentColor" /> < title > { title } <
        / title > < / svg >
    }
}
