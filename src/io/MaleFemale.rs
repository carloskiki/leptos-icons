#[cfg(feature = "IoMaleFemale")]
use leptos::*;
#[cfg(feature = "IoMaleFemale")]
///This icon requires the feature `IoMaleFemale` to be enabled.
#[component]
pub fn MaleFemale(
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
        "M426,16H352a22,22,0,0,0,0,44h20.89l-37.1,37.09A157.68,157.68,0,0,0,216,42C128.88,42,58,112.88,58,200c0,79.66,59.26,145.72,136,156.46V394H166a22,22,0,0,0,0,44h28v36a22,22,0,0,0,44,0V438h28a22,22,0,0,0,0-44H238V356.46c76.74-10.74,136-76.8,136-156.46a157.15,157.15,0,0,0-14-64.92l44-44V112a22,22,0,0,0,44,0V38A22,22,0,0,0,426,16ZM216,314A114,114,0,1,1,330,200,114.13,114.13,0,0,1,216,314Z"
        /> < title > { title } < / title > < / svg >
    }
}
