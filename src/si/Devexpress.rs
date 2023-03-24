#[cfg(feature = "SiDevexpress")]
use leptos::*;
#[cfg(feature = "SiDevexpress")]
///This icon requires the feature `SiDevexpress` to be enabled.
#[component]
pub fn Devexpress(
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
        "M1.4 0h21.2c.8 0 1.4.6 1.4 1.4v1.1c0 .8-.6 1-.9 1C11.7 4.9 4.2 9.7 0 14.1V1.4C0 .6.6 0 1.4 0zm.022 19.567L1.7 19.2C5.3 14.6 12.4 8.3 24 6.3v16.3c0 .8-.6 1.4-1.4 1.4H1.4C.6 24 0 23.4 0 22.6v-.4c0-.3.2-.8.3-.9.252-.589.646-1.107 1.122-1.733z"
        /> < title > { title } < / title > < / svg >
    }
}
