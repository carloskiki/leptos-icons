#[cfg(feature = "ImPrinter")]
use leptos::*;
#[cfg(feature = "ImPrinter")]
///This icon requires the feature `ImPrinter` to be enabled.
#[component]
pub fn Printer(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d = "M4 1h8v2h-8v-2z" />< path xmlns = "http://www.w3.org/2000/svg"
        xmlns : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M15 4h-14c-0.55 0-1 0.45-1 1v5c0 0.55 0.45 1 1 1h3v4h8v-4h3c0.55 0 1-0.45 1-1v-5c0-0.55-0.45-1-1-1zM2 7c-0.552 0-1-0.448-1-1s0.448-1 1-1 1 0.448 1 1-0.448 1-1 1zM11 14h-6v-5h6v5z"
        /> < title > { title } < / title > < / svg >
    }
}
