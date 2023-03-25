#[cfg(feature = "IoCodeDownload")]
use leptos::*;
#[cfg(feature = "IoCodeDownload")]
///This icon requires the feature `IoCodeDownload` to be enabled.
#[component]
pub fn CodeDownload(
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
        points = "160 368 32 256 160 144" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:42px"
        />< polyline xmlns = "http://www.w3.org/2000/svg" points =
        "352 368 480 256 352 144" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:42px"
        />< polyline xmlns = "http://www.w3.org/2000/svg" points =
        "192 288.1 256 352 320 288.1" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:42px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "256" y1 = "160" x2 = "256" y2
        = "336.03" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:42px"
        /> < title > { title } < / title > < / svg >
    }
}
