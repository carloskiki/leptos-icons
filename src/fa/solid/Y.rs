#[cfg(feature = "FaSolidY")]
use leptos::*;
#[cfg(feature = "FaSolidY")]
///This icon requires the feature `FaSolidY` to be enabled.
#[component]
pub fn Y(
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
        "M58 45.4C47.8 31 27.8 27.7 13.4 38S-4.3 68.2 6 82.6L160 298.3V448c0 17.7 14.3 32 32 32s32-14.3 32-32V298.3L378 82.6c10.3-14.4 6.9-34.4-7.4-44.6S336.2 31 326 45.4L192 232.9 58 45.4z"
        /> < title > { title } < / title > < / svg >
    }
}
