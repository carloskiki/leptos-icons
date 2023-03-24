#[cfg(feature = "IoFilterCircleOutline")]
use leptos::*;
#[cfg(feature = "IoFilterCircleOutline")]
///This icon requires the feature `IoFilterCircleOutline` to be enabled.
#[component]
pub fn FilterCircleOutline(
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
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" fill = "none"
        stroke = "#000" stroke - width = "32" stroke - miterlimit = "10" d =
        "M448,256c0-106-86-192-192-192S64,150,64,256s86,192,192,192S448,362,448,256Z" /><
        line xmlns = "http://www.w3.org/2000/svg" fill = "none" stroke = "#000" stroke -
        width = "32" stroke - linecap = "round" stroke - linejoin = "round" x1 = "144" y1
        = "208" x2 = "368" y2 = "208" />< line xmlns = "http://www.w3.org/2000/svg" fill
        = "none" stroke = "#000" stroke - width = "32" stroke - linecap = "round" stroke
        - linejoin = "round" x1 = "176" y1 = "272" x2 = "336" y2 = "272" />< line xmlns =
        "http://www.w3.org/2000/svg" fill = "none" stroke = "#000" stroke - width = "32"
        stroke - linecap = "round" stroke - linejoin = "round" x1 = "224" y1 = "336" x2 =
        "288" y2 = "336" /> < title > { title } < / title > < / svg >
    }
}
