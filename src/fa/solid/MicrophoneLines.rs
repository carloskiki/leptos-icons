#[cfg(feature = "FaSolidMicrophoneLines")]
use leptos::*;
#[cfg(feature = "FaSolidMicrophoneLines")]
///This icon requires the feature `FaSolidMicrophoneLines` to be enabled.
#[component]
pub fn MicrophoneLines(
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
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M80 96V256c0 53 43 96 96 96s96-43 96-96H192c-8.8 0-16-7.2-16-16s7.2-16 16-16h80V192H192c-8.8 0-16-7.2-16-16s7.2-16 16-16h80V128H192c-8.8 0-16-7.2-16-16s7.2-16 16-16h80c0-53-43-96-96-96S80 43 80 96zM304 240v16c0 70.7-57.3 128-128 128s-128-57.3-128-128V216c0-13.3-10.7-24-24-24s-24 10.7-24 24v40c0 89.1 66.2 162.7 152 174.4V464H104c-13.3 0-24 10.7-24 24s10.7 24 24 24h72 72c13.3 0 24-10.7 24-24s-10.7-24-24-24H200V430.4c85.8-11.7 152-85.3 152-174.4V216c0-13.3-10.7-24-24-24s-24 10.7-24 24v24z"
        /> < title > { title } < / title > < / svg >
    }
}
