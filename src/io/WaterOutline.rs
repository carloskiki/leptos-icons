#[cfg(feature = "IoWaterOutline")]
use leptos::*;
#[cfg(feature = "IoWaterOutline")]
///This icon requires the feature `IoWaterOutline` to be enabled.
#[component]
pub fn WaterOutline(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M400,320c0,88.37-55.63,144-144,144S112,408.37,112,320c0-94.83,103.23-222.85,134.89-259.88a12,12,0,0,1,18.23,0C296.77,97.15,400,225.17,400,320Z"
        style = "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M344,328a72,72,0,0,1-72,72" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
