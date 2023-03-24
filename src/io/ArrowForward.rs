#[cfg(feature = "IoArrowForward")]
use leptos::*;
#[cfg(feature = "IoArrowForward")]
///This icon requires the feature `IoArrowForward` to be enabled.
#[component]
pub fn ArrowForward(
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
        "0 0 512 512" width = { size.clone() } height = { size } > < polyline xmlns =
        "http://www.w3.org/2000/svg" points = "268 112 412 256 268 400" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:48px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "392" y1 = "256" x2 = "100" y2
        = "256" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:48px"
        /> < title > { title } < / title > < / svg >
    }
}
