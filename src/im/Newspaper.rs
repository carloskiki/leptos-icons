#[cfg(feature = "ImNewspaper")]
use leptos::*;
#[cfg(feature = "ImNewspaper")]
///This icon requires the feature `ImNewspaper` to be enabled.
#[component]
pub fn Newspaper(
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
        "M14 4v-2h-14v11c0 0.552 0.448 1 1 1h13.5c0.828 0 1.5-0.672 1.5-1.5v-8.5h-2zM13 13h-12v-10h12v10zM2 5h10v1h-10zM8 7h4v1h-4zM8 9h4v1h-4zM8 11h3v1h-3zM2 7h5v5h-5z"
        /> < title > { title } < / title > < / svg >
    }
}
