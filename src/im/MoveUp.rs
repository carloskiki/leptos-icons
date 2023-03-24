#[cfg(feature = "ImMoveUp")]
use leptos::*;
#[cfg(feature = "ImMoveUp")]
///This icon requires the feature `ImMoveUp` to be enabled.
#[component]
pub fn MoveUp(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M11 8v6h1v-6h2.5l-3-3-3 3z" />< path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M1 3h1.5v1h-1.5v-1z" />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink
        = "http://www.w3.org/1999/xlink" fill = "#000000" d = "M3 3h1.5v1h-1.5v-1z" /><
        path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M5 3h1v1.5h-1v-1.5z" /><
        path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M1 6.5h1v1.5h-1v-1.5z" /><
        path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M2.5 7h1.5v1h-1.5v-1z" /><
        path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M4.5 7h1.5v1h-1.5v-1z" /><
        path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M1 4.5h1v1.5h-1v-1.5z" /><
        path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M5 5h1v1.5h-1v-1.5z" /><
        path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M5 11v3h-3v-3h3zM6 10h-5v5h5v-5z" /> < title > { title } < / title > < / svg >
    }
}
