#[cfg(feature = "IoCloudDownloadOutline")]
use leptos::*;
#[cfg(feature = "IoCloudDownloadOutline")]
///This icon requires the feature `IoCloudDownloadOutline` to be enabled.
#[component]
pub fn CloudDownloadOutline(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M320,336h76c55,0,100-21.21,100-75.6s-53-73.47-96-75.6C391.11,99.74,329,48,256,48c-69,0-113.44,45.79-128,91.2-60,5.7-112,35.88-112,98.4S70,336,136,336h56"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< polyline xmlns = "http://www.w3.org/2000/svg" points =
        "192 400.1 256 464 320 400.1" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "256" y1 = "224" x2 = "256" y2
        = "448.03" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
