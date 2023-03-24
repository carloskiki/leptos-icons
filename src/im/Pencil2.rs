#[cfg(feature = "ImPencil2")]
use leptos::*;
#[cfg(feature = "ImPencil2")]
///This icon requires the feature `ImPencil2` to be enabled.
#[component]
pub fn Pencil2(
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
        "M6 10l2-1 7-7-1-1-7 7-1 2zM4.52 13.548c-0.494-1.043-1.026-1.574-2.069-2.069l1.548-4.262 2-1.217 6-6h-3l-6 6-3 10 10-3 6-6v-3l-6 6-1.217 2z"
        /> < title > { title } < / title > < / svg >
    }
}
