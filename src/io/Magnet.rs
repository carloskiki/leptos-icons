#[cfg(feature = "IoMagnet")]
use leptos::*;
#[cfg(feature = "IoMagnet")]
///This icon requires the feature `IoMagnet` to be enabled.
#[component]
pub fn Magnet(
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
        "0 0 512 512" width = { size.clone() } height = { size } > < line xmlns =
        "http://www.w3.org/2000/svg" x1 = "191.98" y1 = "463.79" x2 = "191.98" y2 =
        "415.79" style =
        "stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px" /><
        line xmlns = "http://www.w3.org/2000/svg" x1 = "90.16" y1 = "421.61" x2 = "124.1"
        y2 = "387.67" style =
        "stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px" /><
        line xmlns = "http://www.w3.org/2000/svg" x1 = "47.98" y1 = "319.79" x2 = "95.98"
        y2 = "319.79" style =
        "stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M267.56,312.32l-31.11,31.11a16,16,0,0,0,0,22.63l45.26,45.25a16,16,0,0,0,22.62,0l31.12-31.11a4,4,0,0,0,0-5.66l-62.23-62.22A4,4,0,0,0,267.56,312.32Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M131.8,176.55l-31.11,31.12a16,16,0,0,0,0,22.62l45.25,45.26a16,16,0,0,0,22.63,0l31.11-31.11a4,4,0,0,0,0-5.66l-62.22-62.23A4,4,0,0,0,131.8,176.55Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M428.85,83.28a144,144,0,0,0-203.71-.06l-65.06,65.05a4,4,0,0,0,0,5.66l62.23,62.22a4,4,0,0,0,5.66,0l65-65.05a48,48,0,0,1,68.46.59c18.3,18.92,17.47,49.24-1.14,67.85L295.85,284a4,4,0,0,0,0,5.66l62.22,62.23a4,4,0,0,0,5.66,0l64.08-64.08C484.18,231.47,485.18,139.68,428.85,83.28Z"
        /> < title > { title } < / title > < / svg >
    }
}
