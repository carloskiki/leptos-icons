#[cfg(feature = "IoBarcodeSharp")]
use leptos::*;
#[cfg(feature = "IoBarcodeSharp")]
///This icon requires the feature `IoBarcodeSharp` to be enabled.
#[component]
pub fn BarcodeSharp(
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
        "http://www.w3.org/2000/svg" > < polyline xmlns = "http://www.w3.org/2000/svg"
        points = "400 400.33 448 400 448 112 400 112.33" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:32px"
        />< polyline xmlns = "http://www.w3.org/2000/svg" points =
        "112 112 64 112.33 64 400.33 112 400" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "384" y1 = "192" x2 = "384" y2
        = "320" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "320" y1 = "160" x2 = "320" y2
        = "352" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "256" y1 = "176" x2 = "256" y2
        = "336" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "192" y1 = "160" x2 = "192" y2
        = "352" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "128" y1 = "192" x2 = "128" y2
        = "320" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
