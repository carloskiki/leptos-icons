#[cfg(feature = "FaSolidJ")]
use leptos::*;
#[cfg(feature = "FaSolidJ")]
///This icon requires the feature `FaSolidJ` to be enabled.
#[component]
pub fn J(
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
        "M288 32c17.7 0 32 14.3 32 32V320c0 88.4-71.6 160-160 160S0 408.4 0 320V288c0-17.7 14.3-32 32-32s32 14.3 32 32v32c0 53 43 96 96 96s96-43 96-96V64c0-17.7 14.3-32 32-32z"
        /> < title > { title } < / title > < / svg >
    }
}
