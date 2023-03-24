#[cfg(feature = "FaSolidFaucet")]
use leptos::*;
#[cfg(feature = "FaSolidFaucet")]
///This icon requires the feature `FaSolidFaucet` to be enabled.
#[component]
pub fn Faucet(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M192 96v12L96 96c-17.7 0-32 14.3-32 32s14.3 32 32 32l96-12 31-3.9 1-.1 1 .1 31 3.9 96 12c17.7 0 32-14.3 32-32s-14.3-32-32-32l-96 12V96c0-17.7-14.3-32-32-32s-32 14.3-32 32zM32 256c-17.7 0-32 14.3-32 32v64c0 17.7 14.3 32 32 32H132.1c20.2 29 53.9 48 91.9 48s71.7-19 91.9-48H352c17.7 0 32 14.3 32 32s14.3 32 32 32h64c17.7 0 32-14.3 32-32c0-88.4-71.6-160-160-160H320l-22.6-22.6c-6-6-14.1-9.4-22.6-9.4H256V180.2l-32-4-32 4V224H173.3c-8.5 0-16.6 3.4-22.6 9.4L128 256H32z"
        /> < title > { title } < / title > < / svg >
    }
}
