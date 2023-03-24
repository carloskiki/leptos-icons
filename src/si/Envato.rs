#[cfg(feature = "SiEnvato")]
use leptos::*;
#[cfg(feature = "SiEnvato")]
///This icon requires the feature `SiEnvato` to be enabled.
#[component]
pub fn Envato(
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
        "M20.058 1.043C16.744-2.841 6.018 4.682 6.104 14.38a.459.459 0 0 1-.45.451.459.459 0 0 1-.388-.221 10.387 10.387 0 0 1-.412-7.634.42.42 0 0 0-.712-.412 10.284 10.284 0 0 0-2.784 7.033A10.284 10.284 0 0 0 11.76 23.999c14.635-.332 11.257-19.491 8.298-22.956z"
        /> < title > { title } < / title > < / svg >
    }
}
