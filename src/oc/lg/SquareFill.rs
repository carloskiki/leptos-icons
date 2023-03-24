#[cfg(feature = "OcLgSquareFill")]
use leptos::*;
#[cfg(feature = "OcLgSquareFill")]
///This icon requires the feature `OcLgSquareFill` to be enabled.
#[component]
pub fn SquareFill(
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
        "M7.75 6h8.5c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0 1 16.25 18h-8.5A1.75 1.75 0 0 1 6 16.25v-8.5C6 6.784 6.784 6 7.75 6Z"
        /> < title > { title } < / title > < / svg >
    }
}
