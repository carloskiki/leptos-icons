#[cfg(feature = "IoVideocamOutline")]
use leptos::*;
#[cfg(feature = "IoVideocamOutline")]
///This icon requires the feature `IoVideocamOutline` to be enabled.
#[component]
pub fn VideocamOutline(
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
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M374.79,308.78,457.5,367A16,16,0,0,0,480,352.38V159.62A16,16,0,0,0,457.5,145l-82.71,58.22A16,16,0,0,0,368,216.3v79.4A16,16,0,0,0,374.79,308.78Z"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M268,384H84a52.15,52.15,0,0,1-52-52V180a52.15,52.15,0,0,1,52-52H268.48A51.68,51.68,0,0,1,320,179.52V332A52.15,52.15,0,0,1,268,384Z"
        style = "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /> < title
        > { title } < / title > < / svg >
    }
}
