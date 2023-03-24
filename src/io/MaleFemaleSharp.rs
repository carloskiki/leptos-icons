#[cfg(feature = "IoMaleFemaleSharp")]
use leptos::*;
#[cfg(feature = "IoMaleFemaleSharp")]
///This icon requires the feature `IoMaleFemaleSharp` to be enabled.
#[component]
pub fn MaleFemaleSharp(
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
        "M330,16V60h42.89l-37.1,37.09A157.67,157.67,0,0,0,216,42C128.88,42,58,112.88,58,200c0,79.66,59.26,145.72,136,156.46V394H144v44h50v58h44V438h50V394H238V356.46c76.74-10.74,136-76.8,136-156.46a157.23,157.23,0,0,0-14-64.93l44-44V134h44V16ZM216,314A114,114,0,1,1,330,200,114.13,114.13,0,0,1,216,314Z"
        /> < title > { title } < / title > < / svg >
    }
}
