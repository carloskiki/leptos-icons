#[cfg(feature = "VsLayout")]
use leptos::*;
#[cfg(feature = "VsLayout")]
///This icon requires the feature `VsLayout` to be enabled.
#[component]
pub fn Layout(
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
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 2L2 3V13L3 14H7L8 13V3L7 2H3ZM3 13V3H7V13H3Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M10 3L11 2H14L15 3V6L14 7H11L10 6V3ZM11 3V6H14V3H11Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M10 10L11 9H14L15 10V13L14 14H11L10 13V10ZM11 10V13H14V10H11Z" /> < title > {
        title } < / title > < / svg >
    }
}
