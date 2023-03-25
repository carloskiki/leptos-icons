#[cfg(feature = "IoMagnetSharp")]
use leptos::*;
#[cfg(feature = "IoMagnetSharp")]
///This icon requires the feature `IoMagnetSharp` to be enabled.
#[component]
pub fn MagnetSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < line xmlns = "http://www.w3.org/2000/svg" x1 =
        "191.98" y1 = "463.58" x2 = "191.98" y2 = "415.58" style =
        "stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px" /><
        line xmlns = "http://www.w3.org/2000/svg" x1 = "90.16" y1 = "421.4" x2 = "124.1"
        y2 = "387.46" style =
        "stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px" /><
        line xmlns = "http://www.w3.org/2000/svg" x1 = "47.98" y1 = "319.58" x2 = "95.98"
        y2 = "319.58" style =
        "stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M422.2,89.82a144,144,0,0,0-203.71-.07l-67.88,67.88,67.88,67.89,67.88-67.89a48,48,0,0,1,68.46.59c18.3,18.92,17.48,49.24-1.14,67.86L286.37,293.4l67.88,67.88,66.91-66.91C477.53,238,478.53,146.22,422.2,89.82Z"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "107.29" y = "188.83" width =
        "64" height = "96" transform = "translate(-126.67 167.86) rotate(-45)" />< rect
        xmlns = "http://www.w3.org/2000/svg" x = "243.06" y = "324.59" width = "64"
        height = "96" transform = "translate(-182.9 303.62) rotate(-45)" /> < title > {
        title } < / title > < / svg >
    }
}
