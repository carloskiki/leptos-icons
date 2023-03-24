#[cfg(feature = "TiExport")]
use leptos::*;
#[cfg(feature = "TiExport")]
///This icon requires the feature `TiExport` to be enabled.
#[component]
pub fn Export(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = { size.clone() } height = { size
        } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 16.5v.5c1.691-2.578 3.6-3.953 6-4v3c0 .551.511 1 1.143 1 .364 0 .675-.158.883-.391 1.933-2.029 5.974-6.109 5.974-6.109s-4.041-4.082-5.975-6.137c-.208-.205-.518-.363-.882-.363-.632 0-1.143.447-1.143 1v3c-4.66 0-6 4.871-6 8.5zM5 21h14c.553 0 1-.448 1-1v-6.046c-.664.676-1.364 1.393-2 2.047v2.999h-12v-12h7v-2h-8c-.553 0-1 .448-1 1v14c0 .552.447 1 1 1z"
        /> < title > { title } < / title > < / svg >
    }
}
