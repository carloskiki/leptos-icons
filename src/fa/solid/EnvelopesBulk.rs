#[cfg(feature = "FaSolidEnvelopesBulk")]
use leptos::*;
#[cfg(feature = "FaSolidEnvelopesBulk")]
///This icon requires the feature `FaSolidEnvelopesBulk` to be enabled.
#[component]
pub fn EnvelopesBulk(
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
        stroke_witdh = "0" style = style viewBox = "0 0 576 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M96 0C78.3 0 64 14.3 64 32V224h96V192c0-35.3 28.7-64 64-64H448V32c0-17.7-14.3-32-32-32H96zM224 160c-17.7 0-32 14.3-32 32v32h96c35.3 0 64 28.7 64 64V416H544c17.7 0 32-14.3 32-32V192c0-17.7-14.3-32-32-32H224zm240 64h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H464c-8.8 0-16-7.2-16-16V240c0-8.8 7.2-16 16-16zM32 256c-17.7 0-32 14.3-32 32v13L155.1 415.9c1.4 1 3.1 1.6 4.9 1.6s3.5-.6 4.9-1.6L320 301V288c0-17.7-14.3-32-32-32H32zm288 84.8L184 441.6c-6.9 5.1-15.3 7.9-24 7.9s-17-2.8-24-7.9L0 340.8V480c0 17.7 14.3 32 32 32H288c17.7 0 32-14.3 32-32V340.8z"
        /> < title > { title } < / title > < / svg >
    }
}
