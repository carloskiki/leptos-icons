#[cfg(feature = "ImLock")]
use leptos::*;
#[cfg(feature = "ImLock")]
///This icon requires the feature `ImLock` to be enabled.
#[component]
pub fn Lock(
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
        "M9.25 7h-0.25v-3c0-1.654-1.346-3-3-3h-2c-1.654 0-3 1.346-3 3v3h-0.25c-0.412 0-0.75 0.338-0.75 0.75v7.5c0 0.412 0.338 0.75 0.75 0.75h8.5c0.412 0 0.75-0.338 0.75-0.75v-7.5c0-0.412-0.338-0.75-0.75-0.75zM3 4c0-0.551 0.449-1 1-1h2c0.551 0 1 0.449 1 1v3h-4v-3z"
        /> < title > { title } < / title > < / svg >
    }
}
