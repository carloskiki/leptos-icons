#[cfg(feature = "RiBusinessLineSlideshow4")]
use leptos::*;
#[cfg(feature = "RiBusinessLineSlideshow4")]
///This icon requires the feature `RiBusinessLineSlideshow4` to be enabled.
#[component]
pub fn Slideshow4(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M8.17 3A3.001 3.001 0 0 1 11 1h2c1.306 0 2.417.835 2.83 2H21a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h5.17zM4 5v14h16V5h-4.17A3.001 3.001 0 0 1 13 7h-2a3.001 3.001 0 0 1-2.83-2H4zm7-2a1 1 0 0 0 0 2h2a1 1 0 0 0 0-2h-2zm-1 6l5 3-5 3V9z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
