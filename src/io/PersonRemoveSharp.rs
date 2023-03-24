#[cfg(feature = "IoPersonRemoveSharp")]
use leptos::*;
#[cfg(feature = "IoPersonRemoveSharp")]
///This icon requires the feature `IoPersonRemoveSharp` to be enabled.
#[component]
pub fn PersonRemoveSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < rect xmlns = "http://www.w3.org/2000/svg" x =
        "16" y = "214" width = "144" height = "36" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "288" cy = "144" r = "112" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M288,288c-69.42,0-208,42.88-208,128v64H496V416C496,330.88,357.42,288,288,288Z"
        /> < title > { title } < / title > < / svg >
    }
}
