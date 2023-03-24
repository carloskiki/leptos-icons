#[cfg(feature = "FaSolidBorderNone")]
use leptos::*;
#[cfg(feature = "FaSolidBorderNone")]
///This icon requires the feature `FaSolidBorderNone` to be enabled.
#[component]
pub fn BorderNone(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M32 416a32 32 0 1 0 0 64 32 32 0 1 0 0-64zm96 64a32 32 0 1 0 0-64 32 32 0 1 0 0 64zm0-384a32 32 0 1 0 0-64 32 32 0 1 0 0 64zm0 128a32 32 0 1 0 0 64 32 32 0 1 0 0-64zM320 480a32 32 0 1 0 0-64 32 32 0 1 0 0 64zm0-448a32 32 0 1 0 0 64 32 32 0 1 0 0-64zm0 256a32 32 0 1 0 0-64 32 32 0 1 0 0 64zM224 416a32 32 0 1 0 0 64 32 32 0 1 0 0-64zm0-320a32 32 0 1 0 0-64 32 32 0 1 0 0 64zm0 128a32 32 0 1 0 0 64 32 32 0 1 0 0-64zM416 480a32 32 0 1 0 0-64 32 32 0 1 0 0 64zm0-384a32 32 0 1 0 0-64 32 32 0 1 0 0 64zM32 32a32 32 0 1 0 0 64 32 32 0 1 0 0-64zM416 288a32 32 0 1 0 0-64 32 32 0 1 0 0 64zM32 224a32 32 0 1 0 0 64 32 32 0 1 0 0-64zM224 384a32 32 0 1 0 0-64 32 32 0 1 0 0 64zm192-64a32 32 0 1 0 0 64 32 32 0 1 0 0-64zM32 384a32 32 0 1 0 0-64 32 32 0 1 0 0 64zM416 128a32 32 0 1 0 0 64 32 32 0 1 0 0-64zM32 192a32 32 0 1 0 0-64 32 32 0 1 0 0 64zm192-64a32 32 0 1 0 0 64 32 32 0 1 0 0-64z"
        /> < title > { title } < / title > < / svg >
    }
}
