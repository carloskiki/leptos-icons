#[cfg(feature = "FaSolidMarsStrokeUp")]
use leptos::*;
#[cfg(feature = "FaSolidMarsStrokeUp")]
///This icon requires the feature `FaSolidMarsStrokeUp` to be enabled.
#[component]
pub fn MarsStrokeUp(
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
        stroke_witdh = "0" style = style viewBox = "0 0 320 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M148.7 4.7c6.2-6.2 16.4-6.2 22.6 0l64 64c4.6 4.6 5.9 11.5 3.5 17.4s-8.3 9.9-14.8 9.9H184v24h32c13.3 0 24 10.7 24 24s-10.7 24-24 24H184v24c0 .6 0 1.2-.1 1.8c77 11.6 136.1 78 136.1 158.2c0 88.4-71.6 160-160 160S0 440.4 0 352c0-80.2 59.1-146.7 136.1-158.2c0-.6-.1-1.2-.1-1.8V168H104c-13.3 0-24-10.7-24-24s10.7-24 24-24h32V96H96c-6.5 0-12.3-3.9-14.8-9.9s-1.1-12.9 3.5-17.4l64-64zM256 352A96 96 0 1 0 64 352a96 96 0 1 0 192 0z"
        /> < title > { title } < / title > < / svg >
    }
}
