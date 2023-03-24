#[cfg(feature = "ImNext")]
use leptos::*;
#[cfg(feature = "ImNext")]
///This icon requires the feature `ImNext` to be enabled.
#[component]
pub fn Next(
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
        "M8 0c4.418 0 8 3.582 8 8s-3.582 8-8 8-8-3.582-8-8 3.582-8 8-8zM8 14.5c3.59 0 6.5-2.91 6.5-6.5s-2.91-6.5-6.5-6.5-6.5 2.91-6.5 6.5 2.91 6.5 6.5 6.5z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M9 8l-4-3v6z" />< path xmlns
        = "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink"
        fill = "#000000" d = "M11 5h-2v6h2v-6z" /> < title > { title } < / title > < /
        svg >
    }
}
