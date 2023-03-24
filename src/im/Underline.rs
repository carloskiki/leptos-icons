#[cfg(feature = "ImUnderline")]
use leptos::*;
#[cfg(feature = "ImUnderline")]
///This icon requires the feature `ImUnderline` to be enabled.
#[component]
pub fn Underline(
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
        = "#000000" d =
        "M11 1h2v6.5c0 2.485-2.239 4.5-5 4.5s-5-2.015-5-4.5v-6.5h2v6.5c0 0.628 0.285 1.23 0.802 1.695 0.577 0.519 1.357 0.805 2.198 0.805s1.621-0.286 2.198-0.805c0.517-0.466 0.802-1.068 0.802-1.695v-6.5zM3 13h10v2h-10z"
        /> < title > { title } < / title > < / svg >
    }
}
