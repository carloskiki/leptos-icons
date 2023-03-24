#[cfg(feature = "OcSmPersonAdd")]
use leptos::*;
#[cfg(feature = "OcSmPersonAdd")]
///This icon requires the feature `OcSmPersonAdd` to be enabled.
#[component]
pub fn PersonAdd(
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
        "M7.9 8.548h-.001a5.528 5.528 0 0 1 3.1 4.659.75.75 0 1 1-1.498.086A4.01 4.01 0 0 0 5.5 9.5a4.01 4.01 0 0 0-4.001 3.793.75.75 0 1 1-1.498-.085 5.527 5.527 0 0 1 3.1-4.66 3.5 3.5 0 1 1 4.799 0ZM13.25 0a.75.75 0 0 1 .75.75V2h1.25a.75.75 0 0 1 0 1.5H14v1.25a.75.75 0 0 1-1.5 0V3.5h-1.25a.75.75 0 0 1 0-1.5h1.25V.75a.75.75 0 0 1 .75-.75ZM5.5 4a2 2 0 1 0-.001 3.999A2 2 0 0 0 5.5 4Z"
        /> < title > { title } < / title > < / svg >
    }
}
