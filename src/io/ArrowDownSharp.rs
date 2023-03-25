#[cfg(feature = "IoArrowDownSharp")]
use leptos::*;
#[cfg(feature = "IoArrowDownSharp")]
///This icon requires the feature `IoArrowDownSharp` to be enabled.
#[component]
pub fn ArrowDownSharp(
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
        "http://www.w3.org/2000/svg" > < polyline xmlns = "http://www.w3.org/2000/svg"
        points = "112 268 256 412 400 268" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:48px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "256" y1 = "392" x2 = "256" y2
        = "100" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:48px"
        /> < title > { title } < / title > < / svg >
    }
}
