#[cfg(feature = "FaSolidCableCar")]
use leptos::*;
#[cfg(feature = "FaSolidCableCar")]
///This icon requires the feature `FaSolidCableCar` to be enabled.
#[component]
pub fn CableCar(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M288 0a32 32 0 1 1 0 64 32 32 0 1 1 0-64zM160 56a32 32 0 1 1 64 0 32 32 0 1 1 -64 0zM32 288c0-35.3 28.7-64 64-64H232V157.5l-203.1 42c-13 2.7-25.7-5.7-28.4-18.6s5.7-25.7 18.6-28.4l232-48 232-48c13-2.7 25.7 5.7 28.4 18.6s-5.7 25.7-18.6 28.4L280 147.5V224H416c35.3 0 64 28.7 64 64V448c0 35.3-28.7 64-64 64H96c-35.3 0-64-28.7-64-64V288zm64 0c-8.8 0-16 7.2-16 16v64c0 8.8 7.2 16 16 16h64c8.8 0 16-7.2 16-16V304c0-8.8-7.2-16-16-16H96zm112 16v64c0 8.8 7.2 16 16 16h64c8.8 0 16-7.2 16-16V304c0-8.8-7.2-16-16-16H224c-8.8 0-16 7.2-16 16zm144-16c-8.8 0-16 7.2-16 16v64c0 8.8 7.2 16 16 16h64c8.8 0 16-7.2 16-16V304c0-8.8-7.2-16-16-16H352z"
        /> < title > { title } < / title > < / svg >
    }
}
