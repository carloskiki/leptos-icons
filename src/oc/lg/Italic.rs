#[cfg(feature = "OcLgItalic")]
use leptos::*;
#[cfg(feature = "OcLgItalic")]
///This icon requires the feature `OcLgItalic` to be enabled.
#[component]
pub fn Italic(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M10 4.75a.75.75 0 0 1 .75-.75h8.5a.75.75 0 0 1 0 1.5h-3.514l-5.828 13h3.342a.75.75 0 0 1 0 1.5h-8.5a.75.75 0 0 1 0-1.5h3.514l5.828-13H10.75a.75.75 0 0 1-.75-.75Z"
        /> < title > { title } < / title > < / svg >
    }
}
