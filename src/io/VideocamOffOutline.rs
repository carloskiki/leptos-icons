#[cfg(feature = "IoVideocamOffOutline")]
use leptos::*;
#[cfg(feature = "IoVideocamOffOutline")]
///This icon requires the feature `IoVideocamOffOutline` to be enabled.
#[component]
pub fn VideocamOffOutline(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" fill = "none" stroke = "#000" stroke - linecap =
        "round" stroke - width = "32" stroke - linejoin = "round" d =
        "M374.79,308.78,457.5,367A16,16,0,0,0,480,352.38V159.62A16,16,0,0,0,457.5,145l-82.71,58.22A16,16,0,0,0,368,216.3v79.4A16,16,0,0,0,374.79,308.78Z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill = "none" stroke = "#000"
        stroke - linecap = "round" stroke - width = "32" stroke - miterlimit = "10" d =
        "M50.19,140.57A51.94,51.94,0,0,0,32,180V332a52.15,52.15,0,0,0,52,52H268a51.6,51.6,0,0,0,22-4.9"
        />< path xmlns = "http://www.w3.org/2000/svg" fill = "none" stroke = "#000"
        stroke - linecap = "round" stroke - width = "32" stroke - miterlimit = "10" d =
        "M208,128h60.48A51.68,51.68,0,0,1,320,179.52V248" />< line xmlns =
        "http://www.w3.org/2000/svg" fill = "none" stroke = "#000" stroke - linecap =
        "round" stroke - width = "32" stroke - miterlimit = "10" x1 = "416" y1 = "416" x2
        = "80" y2 = "80" /> < title > { title } < / title > < / svg >
    }
}
