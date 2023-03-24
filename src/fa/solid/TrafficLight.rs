#[cfg(feature = "FaSolidTrafficLight")]
use leptos::*;
#[cfg(feature = "FaSolidTrafficLight")]
///This icon requires the feature `FaSolidTrafficLight` to be enabled.
#[component]
pub fn TrafficLight(
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
        stroke_witdh = "0" style = style viewBox = "0 0 320 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M64 0C28.7 0 0 28.7 0 64V352c0 88.4 71.6 160 160 160s160-71.6 160-160V64c0-35.3-28.7-64-64-64H64zm96 416a48 48 0 1 1 0-96 48 48 0 1 1 0 96zm48-176a48 48 0 1 1 -96 0 48 48 0 1 1 96 0zm-48-80a48 48 0 1 1 0-96 48 48 0 1 1 0 96z"
        /> < title > { title } < / title > < / svg >
    }
}
