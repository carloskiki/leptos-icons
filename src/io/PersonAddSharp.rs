#[cfg(feature = "IoPersonAddSharp")]
use leptos::*;
#[cfg(feature = "IoPersonAddSharp")]
///This icon requires the feature `IoPersonAddSharp` to be enabled.
#[component]
pub fn PersonAddSharp(
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
        "0 0 512 512" width = { size.clone() } height = { size } > < polygon xmlns =
        "http://www.w3.org/2000/svg" points =
        "106 304 106 250 160 250 160 214 106 214 106 160 70 160 70 214 16 214 16 250 70 250 70 304 106 304"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "288" cy = "144" r = "112"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M288,288c-69.42,0-208,42.88-208,128v64H496V416C496,330.88,357.42,288,288,288Z"
        /> < title > { title } < / title > < / svg >
    }
}
