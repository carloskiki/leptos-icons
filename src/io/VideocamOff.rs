#[cfg(feature = "IoVideocamOff")]
use leptos::*;
#[cfg(feature = "IoVideocamOff")]
///This icon requires the feature `IoVideocamOff` to be enabled.
#[component]
pub fn VideocamOff(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M336,179.52A67.52,67.52,0,0,0,268.48,112h-79.2a4,4,0,0,0-2.82,6.83L329.17,261.54a4,4,0,0,0,6.83-2.82Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16,180V332a68,68,0,0,0,68,68H268a67.66,67.66,0,0,0,42.84-15.24,4,4,0,0,0,.33-6L54.41,122a4,4,0,0,0-4.87-.62A68,68,0,0,0,16,180Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M464,384.39a32,32,0,0,1-13-2.77,15.77,15.77,0,0,1-2.71-1.54l-82.71-58.22h0A32,32,0,0,1,352,295.7V216.3a32,32,0,0,1,13.58-26.16l82.71-58.22a15.77,15.77,0,0,1,2.71-1.54,32,32,0,0,1,45,29.24V352.38a32,32,0,0,1-32,32Z"
        />< line xmlns = "http://www.w3.org/2000/svg" fill = "none" stroke = "#000"
        stroke - linecap = "round" stroke - miterlimit = "10" stroke - width = "32" x1 =
        "416" y1 = "416" x2 = "80" y2 = "80" /> < title > { title } < / title > < / svg >
    }
}
