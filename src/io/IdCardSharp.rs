#[cfg(feature = "IoIdCardSharp")]
use leptos::*;
#[cfg(feature = "IoIdCardSharp")]
///This icon requires the feature `IoIdCardSharp` to be enabled.
#[component]
pub fn IdCardSharp(
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
        "http://www.w3.org/2000/svg" d =
        "M408,16H104A24,24,0,0,0,80,40V472a24,24,0,0,0,24,24H408a24,24,0,0,0,24-24V40A24,24,0,0,0,408,16ZM346.9,312.77a43,43,0,1,1-40.71-40.71A43,43,0,0,1,346.9,312.77ZM192,64H320V96H192ZM384,448H224V423.4c0-32.72,53.27-49.21,80-49.21s80,16.49,80,49.21Z"
        /> < title > { title } < / title > < / svg >
    }
}
