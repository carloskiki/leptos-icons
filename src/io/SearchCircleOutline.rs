#[cfg(feature = "IoSearchCircleOutline")]
use leptos::*;
#[cfg(feature = "IoSearchCircleOutline")]
///This icon requires the feature `IoSearchCircleOutline` to be enabled.
#[component]
pub fn SearchCircleOutline(
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
        "M256,80A176,176,0,1,0,432,256,176,176,0,0,0,256,80Z" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M232,160a72,72,0,1,0,72,72A72,72,0,0,0,232,160Z" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "283.64" y1 = "283.64" x2 = "336" y2 = "336"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
