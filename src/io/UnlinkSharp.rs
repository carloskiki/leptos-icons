#[cfg(feature = "IoUnlinkSharp")]
use leptos::*;
#[cfg(feature = "IoUnlinkSharp")]
///This icon requires the feature `IoUnlinkSharp` to be enabled.
#[component]
pub fn UnlinkSharp(
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
        stroke_witdh = "0" style = style id = "icons" viewBox = "0 0 512 512" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d = "M200.66,352H144a96,96,0,0,1,0-192h55.41" fill =
        "none" stroke = "#000" stroke - linecap = "square" stroke - linejoin = "round"
        stroke - width = "48" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M312.59,160H368a96,96,0,0,1,0,192H311.34" fill = "none" stroke = "#000" stroke -
        linecap = "square" stroke - linejoin = "round" stroke - width = "48" /> < title >
        { title } < / title > < / svg >
    }
}
