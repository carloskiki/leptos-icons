#[cfg(feature = "IoMagnetOutline")]
use leptos::*;
#[cfg(feature = "IoMagnetOutline")]
///This icon requires the feature `IoMagnetOutline` to be enabled.
#[component]
pub fn MagnetOutline(
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
        "M421.83,293.82A144,144,0,0,0,218.18,90.17" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M353.94,225.94a48,48,0,0,0-67.88-67.88" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "192" y1 = "464" x2 = "192" y2 = "416" style =
        "stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px" /><
        line xmlns = "http://www.w3.org/2000/svg" x1 = "90.18" y1 = "421.82" x2 =
        "124.12" y2 = "387.88" style =
        "stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px" /><
        line xmlns = "http://www.w3.org/2000/svg" x1 = "48" y1 = "320" x2 = "96" y2 =
        "320" style =
        "stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M286.06,158.06,172.92,271.19a32,32,0,0,1-45.25,0L105,248.57a32,32,0,0,1,0-45.26L218.18,90.17"
        style = "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M421.83,293.82,308.69,407a32,32,0,0,1-45.26,0l-22.62-22.63a32,32,0,0,1,0-45.26L353.94,225.94"
        style = "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< line
        xmlns = "http://www.w3.org/2000/svg" x1 = "139.6" y1 = "169.98" x2 = "207.48" y2
        = "237.87" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "275.36" y1 = "305.75" x2 = "343.25" y2 =
        "373.63" style = "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
