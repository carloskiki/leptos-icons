#[cfg(feature = "IoWifiSharp")]
use leptos::*;
#[cfg(feature = "IoWifiSharp")]
///This icon requires the feature `IoWifiSharp` to be enabled.
#[component]
pub fn WifiSharp(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d = "M332.69,320a115,115,0,0,0-152.8,0" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:42px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M393.74,259a201.26,201.26,0,0,0-274.92,0" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:42px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M448,191.52a288,288,0,0,0-383.44,0" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:42px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M300.67,384,256,433l-44.34-49a56.73,56.73,0,0,1,88.92,0Z" /> < title > { title }
        < / title > < / svg >
    }
}
