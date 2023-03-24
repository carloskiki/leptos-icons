#[cfg(feature = "SiAlitalia")]
use leptos::*;
#[cfg(feature = "SiAlitalia")]
///This icon requires the feature `SiAlitalia` to be enabled.
#[component]
pub fn Alitalia(
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
        "M5.429 20.551H0l14.257-14.87c1.622-1.765 2.878-2.232 4.686-2.232H24L21.602 20.55h-4.17L19.49 5.907M15.7 20.551l1.384-9.842-9.457 9.842Z"
        /> < title > { title } < / title > < / svg >
    }
}
