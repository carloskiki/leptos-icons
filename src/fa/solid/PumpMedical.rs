#[cfg(feature = "FaSolidPumpMedical")]
use leptos::*;
#[cfg(feature = "FaSolidPumpMedical")]
///This icon requires the feature `FaSolidPumpMedical` to be enabled.
#[component]
pub fn PumpMedical(
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
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M96 32v96H224V96h60.1c4.2 0 8.3 1.7 11.3 4.7l33.9 33.9c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L340.7 55.4c-15-15-35.4-23.4-56.6-23.4H224c0-17.7-14.3-32-32-32H128C110.3 0 96 14.3 96 32zM85.4 160c-33.3 0-61 25.5-63.8 58.7L3 442.7C-.1 480 29.3 512 66.8 512H253.2c37.4 0 66.9-32 63.8-69.3l-18.7-224c-2.8-33.2-30.5-58.7-63.8-58.7H85.4zM184 280v32h32c13.3 0 24 10.7 24 24s-10.7 24-24 24H184v32c0 13.3-10.7 24-24 24s-24-10.7-24-24V360H104c-13.3 0-24-10.7-24-24s10.7-24 24-24h32V280c0-13.3 10.7-24 24-24s24 10.7 24 24z"
        /> < title > { title } < / title > < / svg >
    }
}
