#[cfg(feature = "OcSmRelFilePath")]
use leptos::*;
#[cfg(feature = "OcSmRelFilePath")]
///This icon requires the feature `OcSmRelFilePath` to be enabled.
#[component]
pub fn RelFilePath(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M13.94 3.045a.75.75 0 0 0-1.38-.59l-4.5 10.5a.75.75 0 1 0 1.38.59l4.5-10.5ZM5 11.5a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Z"
        /> < title > { title } < / title > < / svg >
    }
}
