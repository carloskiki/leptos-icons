#[cfg(feature = "FaSolidPumpSoap")]
use leptos::*;
#[cfg(feature = "FaSolidPumpSoap")]
///This icon requires the feature `FaSolidPumpSoap` to be enabled.
#[component]
pub fn PumpSoap(
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
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M96 32v96H224V96h60.1c4.2 0 8.3 1.7 11.3 4.7l33.9 33.9c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L340.7 55.4c-15-15-35.4-23.4-56.6-23.4H224c0-17.7-14.3-32-32-32H128C110.3 0 96 14.3 96 32zM85.4 160c-33.3 0-61 25.5-63.8 58.7L3 442.7C-.1 480 29.3 512 66.8 512H253.2c37.4 0 66.9-32 63.8-69.3l-18.7-224c-2.8-33.2-30.5-58.7-63.8-58.7H85.4zM224 360c0 35.3-28.7 56-64 56s-64-20.7-64-56c0-32.5 37-80.9 50.9-97.9c3.2-3.9 8.1-6.1 13.1-6.1s9.9 2.2 13.1 6.1C187 279.1 224 327.5 224 360z"
        /> < title > { title } < / title > < / svg >
    }
}
