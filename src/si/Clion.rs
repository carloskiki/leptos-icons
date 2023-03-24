#[cfg(feature = "SiClion")]
use leptos::*;
#[cfg(feature = "SiClion")]
///This icon requires the feature `SiClion` to be enabled.
#[component]
pub fn Clion(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M0 0v24h24V0H0zm7.041 3a5.049 5.049 0 0 1 .219 0c1.86 0 3 .6 3.9 1.56L9.78 6.18C9 5.46 8.22 5.04 7.26 5.04c-1.68 0-2.88 1.38-2.88 3.12 0 1.68 1.2 3.12 2.88 3.12 1.14 0 1.86-.48 2.64-1.14l1.38 1.38c-1.02 1.08-2.16 1.8-4.08 1.8a5.1 5.1 0 0 1-5.1-5.16A5.049 5.049 0 0 1 7.04 3zm5.738.12H15v8.1h4.32v1.86h-6.54V3.12zM2.28 19.5h9V21h-9v-1.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
