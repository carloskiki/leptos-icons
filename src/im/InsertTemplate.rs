#[cfg(feature = "ImInsertTemplate")]
use leptos::*;
#[cfg(feature = "ImInsertTemplate")]
///This icon requires the feature `ImInsertTemplate` to be enabled.
#[component]
pub fn InsertTemplate(
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
        "M6 3h2v1h-2zM9 3h2v1h-2zM14 3v4h-3v-1h2v-2h-1v-1zM5 6h2v1h-2zM8 6h2v1h-2zM3 4v2h1v1h-2v-4h3v1zM6 9h2v1h-2zM9 9h2v1h-2zM14 9v4h-3v-1h2v-2h-1v-1zM5 12h2v1h-2zM8 12h2v1h-2zM3 10v2h1v1h-2v-4h3v1zM15 1h-14v14h14v-14zM16 0v0 16h-16v-16h16z"
        /> < title > { title } < / title > < / svg >
    }
}
