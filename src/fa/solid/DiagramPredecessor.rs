#[cfg(feature = "FaSolidDiagramPredecessor")]
use leptos::*;
#[cfg(feature = "FaSolidDiagramPredecessor")]
///This icon requires the feature `FaSolidDiagramPredecessor` to be enabled.
#[component]
pub fn DiagramPredecessor(
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
        "M448 416l0-64L64 352l0 64 384 0zm0 64L64 480c-35.3 0-64-28.7-64-64l0-64c0-35.3 28.7-64 64-64l384 0c35.3 0 64 28.7 64 64l0 64c0 35.3-28.7 64-64 64zM288 160c0 35.3-28.7 64-64 64L64 224c-35.3 0-64-28.7-64-64L0 96C0 60.7 28.7 32 64 32l144 0 16 0 144 0c44.2 0 80 35.8 80 80l0 16 38.1 0c21.4 0 32.1 25.9 17 41L433 239c-9.4 9.4-24.6 9.4-33.9 0L329 169c-15.1-15.1-4.4-41 17-41l38.1 0 0-16c0-8.8-7.2-16-16-16l-80 0 0 64z"
        /> < title > { title } < / title > < / svg >
    }
}
