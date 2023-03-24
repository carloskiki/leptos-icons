#[cfg(feature = "IoDesktopOutline")]
use leptos::*;
#[cfg(feature = "IoDesktopOutline")]
///This icon requires the feature `IoDesktopOutline` to be enabled.
#[component]
pub fn DesktopOutline(
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
        "0 0 512 512" width = { size.clone() } height = { size } > < rect xmlns =
        "http://www.w3.org/2000/svg" x = "32" y = "64" width = "448" height = "320" rx =
        "32" ry = "32" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< polygon xmlns
        = "http://www.w3.org/2000/svg" points = "304 448 296 384 216 384 208 448 304 448"
        style =
        "stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><
        line xmlns = "http://www.w3.org/2000/svg" x1 = "368" y1 = "448" x2 = "144" y2 =
        "448" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M32,304v48a32.09,32.09,0,0,0,32,32H448a32.09,32.09,0,0,0,32-32V304Zm224,64a16,16,0,1,1,16-16A16,16,0,0,1,256,368Z"
        /> < title > { title } < / title > < / svg >
    }
}
