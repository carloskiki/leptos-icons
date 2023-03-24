#[cfg(feature = "ImDatabase")]
use leptos::*;
#[cfg(feature = "ImDatabase")]
///This icon requires the feature `ImDatabase` to be enabled.
#[component]
pub fn Database(
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
        "M8 0c-4.418 0-8 1.119-8 2.5v2c0 1.381 3.582 2.5 8 2.5s8-1.119 8-2.5v-2c0-1.381-3.582-2.5-8-2.5z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M8 8.5c-4.418 0-8-1.119-8-2.5v3c0 1.381 3.582 2.5 8 2.5s8-1.119 8-2.5v-3c0 1.381-3.582 2.5-8 2.5z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M8 13c-4.418 0-8-1.119-8-2.5v3c0 1.381 3.582 2.5 8 2.5s8-1.119 8-2.5v-3c0 1.381-3.582 2.5-8 2.5z"
        /> < title > { title } < / title > < / svg >
    }
}
