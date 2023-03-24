#[cfg(feature = "ImUngroup")]
use leptos::*;
#[cfg(feature = "ImUngroup")]
///This icon requires the feature `ImUngroup` to be enabled.
#[component]
pub fn Ungroup(
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
        "M6 7.25c0 0.412-0.338 0.75-0.75 0.75h-1.5c-0.413 0-0.75-0.338-0.75-0.75v-1.5c0-0.412 0.337-0.75 0.75-0.75h1.5c0.412 0 0.75 0.338 0.75 0.75v1.5z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M11 7.25c0 0.412-0.338 0.75-0.75 0.75h-1.5c-0.412 0-0.75-0.338-0.75-0.75v-1.5c0-0.412 0.338-0.75 0.75-0.75h1.5c0.412 0 0.75 0.338 0.75 0.75v1.5z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M6 12.25c0 0.412-0.338 0.75-0.75 0.75h-1.5c-0.413 0-0.75-0.338-0.75-0.75v-1.5c0-0.412 0.337-0.75 0.75-0.75h1.5c0.412 0 0.75 0.338 0.75 0.75v1.5z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M11 12.25c0 0.412-0.338 0.75-0.75 0.75h-1.5c-0.412 0-0.75-0.338-0.75-0.75v-1.5c0-0.412 0.338-0.75 0.75-0.75h1.5c0.412 0 0.75 0.338 0.75 0.75v1.5z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M14.251 2.5l1.749-1.749v-0.751h-0.751l-1.749 1.749-1.749-1.749h-0.751v0.751l1.749 1.749-1.749 1.749v0.751h0.751l1.749-1.749 1.749 1.749h0.751v-0.751z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M0 12h1v2h-1v-2z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M0 9h1v2h-1v-2z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M13 7h1v2h-1v-2z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M13 13h1v2h-1v-2z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M13 10h1v2h-1v-2z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M0 6h1v2h-1v-2z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M0 3h1v2h-1v-2z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M8 2h2v1h-2v-1z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M5 2h2v1h-2v-1z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M2 2h2v1h-2v-1z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M7 15h2v1h-2v-1z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M10 15h2v1h-2v-1z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M4 15h2v1h-2v-1z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M1 15h2v1h-2v-1z" /> < title
        > { title } < / title > < / svg >
    }
}
