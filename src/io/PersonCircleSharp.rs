#[cfg(feature = "IoPersonCircleSharp")]
use leptos::*;
#[cfg(feature = "IoPersonCircleSharp")]
///This icon requires the feature `IoPersonCircleSharp` to be enabled.
#[component]
pub fn PersonCircleSharp(
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
        "M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48Zm2,96a72,72,0,1,1-72,72A72,72,0,0,1,258,144Zm-2,288a175.55,175.55,0,0,1-129.18-56.6C135.66,329.62,215.06,320,256,320s120.34,9.62,129.18,55.39A175.52,175.52,0,0,1,256,432Z"
        /> < title > { title } < / title > < / svg >
    }
}
