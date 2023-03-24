#[cfg(feature = "IoCheckmarkDoneCircleSharp")]
use leptos::*;
#[cfg(feature = "IoCheckmarkDoneCircleSharp")]
///This icon requires the feature `IoCheckmarkDoneCircleSharp` to be enabled.
#[component]
pub fn CheckmarkDoneCircleSharp(
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
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48Zm48.19,121.42,24.1,21.06-73.61,84.1-24.1-23.06ZM191.93,342.63,121.37,272,144,249.37,214.57,320Zm65,.79L185.55,272l22.64-22.62,47.16,47.21L366.48,169.42l24.1,21.06Z"
        /> < title > { title } < / title > < / svg >
    }
}
