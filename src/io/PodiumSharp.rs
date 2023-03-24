#[cfg(feature = "IoPodiumSharp")]
use leptos::*;
#[cfg(feature = "IoPodiumSharp")]
///This icon requires the feature `IoPodiumSharp` to be enabled.
#[component]
pub fn PodiumSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < rect xmlns =
        "http://www.w3.org/2000/svg" x = "160" y = "32" width = "192" height = "448" /><
        rect xmlns = "http://www.w3.org/2000/svg" x = "384" y = "192" width = "112"
        height = "288" />< rect xmlns = "http://www.w3.org/2000/svg" x = "16" y = "128"
        width = "112" height = "352" /> < title > { title } < / title > < / svg >
    }
}
