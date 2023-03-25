#[cfg(feature = "IoWalkSharp")]
use leptos::*;
#[cfg(feature = "IoWalkSharp")]
///This icon requires the feature `IoWalkSharp` to be enabled.
#[component]
pub fn WalkSharp(
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
        "M315.09,481.38,258.14,366.26l-45-57.56a73.11,73.11,0,0,1-10.16-37.17V142h15.73A40.36,40.36,0,0,1,259,182.32V344.84"
        style =
        "stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:32px" /><
        polyline xmlns = "http://www.w3.org/2000/svg" points =
        "128.18 291.5 128.18 216.73 193.13 151.63" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:32px"
        />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "376.35 295.73 292.4 239.35 292.4 194.67 397.08 267.62 376.35 295.73" />< polygon
        xmlns = "http://www.w3.org/2000/svg" points =
        "175.13 498.58 153.7 471.67 234.03 390.13 249.56 422.2 175.13 498.58" />< circle
        xmlns = "http://www.w3.org/2000/svg" cx = "259.02" cy = "67.21" r = "37.38" style
        = "stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:16px" />
        < title > { title } < / title > < / svg >
    }
}
