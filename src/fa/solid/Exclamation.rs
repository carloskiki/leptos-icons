#[cfg(feature = "FaSolidExclamation")]
use leptos::*;
#[cfg(feature = "FaSolidExclamation")]
///This icon requires the feature `FaSolidExclamation` to be enabled.
#[component]
pub fn Exclamation(
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
        stroke_witdh = "0" style = style viewBox = "0 0 128 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M72 64c0-17.7-14.3-32-32-32S8 46.3 8 64V320c0 17.7 14.3 32 32 32s32-14.3 32-32V64zM40 480a40 40 0 1 0 0-80 40 40 0 1 0 0 80z"
        /> < title > { title } < / title > < / svg >
    }
}
