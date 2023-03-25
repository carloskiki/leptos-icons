#[cfg(feature = "IoGridSharp")]
use leptos::*;
#[cfg(feature = "IoGridSharp")]
///This icon requires the feature `IoGridSharp` to be enabled.
#[component]
pub fn GridSharp(
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
        "M240,240H32V32H240Z" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M480,240H272V32H480Z" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M240,480H32V272H240Z" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M480,480H272V272H480Z" /> < title > { title } < / title > < / svg >
    }
}
