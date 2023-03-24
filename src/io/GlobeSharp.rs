#[cfg(feature = "IoGlobeSharp")]
use leptos::*;
#[cfg(feature = "IoGlobeSharp")]
///This icon requires the feature `IoGlobeSharp` to be enabled.
#[component]
pub fn GlobeSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,48C141.13,48,48,141.13,48,256s93.13,208,208,208,208-93.13,208-208S370.87,48,256,48Z"
        style = "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:44px" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M256,48c-58.07,0-112.67,93.13-112.67,208S197.93,464,256,464s112.67-93.13,112.67-208S314.07,48,256,48Z"
        style = "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:44px" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M117.33,121.33c38.24,27.15,86.38,43.34,138.67,43.34s100.43-16.19,138.67-43.34"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:44px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M394.67,390.67c-38.24-27.15-86.38-43.34-138.67-43.34s-100.43,16.19-138.67,43.34"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:44px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "256" y1 = "48" x2 = "256" y2
        = "464" style = "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:44px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "464" y1 = "256" x2 = "48" y2
        = "256" style = "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:44px" />
        < title > { title } < / title > < / svg >
    }
}
