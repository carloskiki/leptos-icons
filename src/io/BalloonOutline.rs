#[cfg(feature = "IoBalloonOutline")]
use leptos::*;
#[cfg(feature = "IoBalloonOutline")]
///This icon requires the feature `IoBalloonOutline` to be enabled.
#[component]
pub fn BalloonOutline(
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
        stroke_witdh = "0" style = style id = "icons" viewBox = "0 0 512 512" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M414.11,153.82C429.66,264.4,345.85,357.09,282.54,366s-169.48-57.5-185-167.68a159.82,159.82,0,1,1,316.53-44.49Z"
        fill = "none" stroke = "#000" stroke - miterlimit = "10" stroke - width = "32"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M236.06,308.05c-32.83-13-67.08-43.1-82.27-85.46" fill = "none" stroke = "#000"
        stroke - linecap = "round" stroke - miterlimit = "10" stroke - width = "32" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M367.7,495.78c-32.83-13-63.31-40.06-78.5-82.41" fill = "none" stroke = "#000"
        stroke - linecap = "round" stroke - miterlimit = "10" stroke - width = "32" /><
        polygon xmlns = "http://www.w3.org/2000/svg" points =
        "266.71 368.21 257.54 417.82 320.85 408.92 298.36 363.76 266.71 368.21" fill =
        "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin = "round"
        stroke - width = "32" /> < title > { title } < / title > < / svg >
    }
}
