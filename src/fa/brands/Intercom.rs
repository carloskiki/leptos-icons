#[cfg(feature = "FaBrandsIntercom")]
use leptos::*;
#[cfg(feature = "FaBrandsIntercom")]
///This icon requires the feature `FaBrandsIntercom` to be enabled.
#[component]
pub fn Intercom(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M392 32H56C25.1 32 0 57.1 0 88v336c0 30.9 25.1 56 56 56h336c30.9 0 56-25.1 56-56V88c0-30.9-25.1-56-56-56zm-108.3 82.1c0-19.8 29.9-19.8 29.9 0v199.5c0 19.8-29.9 19.8-29.9 0V114.1zm-74.6-7.5c0-19.8 29.9-19.8 29.9 0v216.5c0 19.8-29.9 19.8-29.9 0V106.6zm-74.7 7.5c0-19.8 29.9-19.8 29.9 0v199.5c0 19.8-29.9 19.8-29.9 0V114.1zM59.7 144c0-19.8 29.9-19.8 29.9 0v134.3c0 19.8-29.9 19.8-29.9 0V144zm323.4 227.8c-72.8 63-241.7 65.4-318.1 0-15-12.8 4.4-35.5 19.4-22.7 65.9 55.3 216.1 53.9 279.3 0 14.9-12.9 34.3 9.8 19.4 22.7zm5.2-93.5c0 19.8-29.9 19.8-29.9 0V144c0-19.8 29.9-19.8 29.9 0v134.3z"
        /> < title > { title } < / title > < / svg >
    }
}
