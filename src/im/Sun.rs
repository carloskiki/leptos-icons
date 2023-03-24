#[cfg(feature = "ImSun")]
use leptos::*;
#[cfg(feature = "ImSun")]
///This icon requires the feature `ImSun` to be enabled.
#[component]
pub fn Sun(
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
        "M8 13c0.552 0 1 0.448 1 1v1c0 0.552-0.448 1-1 1s-1-0.448-1-1v-1c0-0.552 0.448-1 1-1zM8 3c-0.552 0-1-0.448-1-1v-1c0-0.552 0.448-1 1-1s1 0.448 1 1v1c0 0.552-0.448 1-1 1zM15 7c0.552 0 1 0.448 1 1s-0.448 1-1 1h-1c-0.552 0-1-0.448-1-1s0.448-1 1-1h1zM3 8c0 0.552-0.448 1-1 1h-1c-0.552 0-1-0.448-1-1s0.448-1 1-1h1c0.552 0 1 0.448 1 1zM12.95 11.536l0.707 0.707c0.39 0.39 0.39 1.024 0 1.414s-1.024 0.39-1.414 0l-0.707-0.707c-0.39-0.39-0.39-1.024 0-1.414s1.024-0.39 1.414 0zM3.050 4.464l-0.707-0.707c-0.391-0.391-0.391-1.024 0-1.414s1.024-0.391 1.414 0l0.707 0.707c0.391 0.391 0.391 1.024 0 1.414s-1.024 0.391-1.414 0zM12.95 4.464c-0.39 0.391-1.024 0.391-1.414 0s-0.39-1.024 0-1.414l0.707-0.707c0.39-0.391 1.024-0.391 1.414 0s0.39 1.024 0 1.414l-0.707 0.707zM3.050 11.536c0.39-0.39 1.024-0.39 1.414 0s0.391 1.024 0 1.414l-0.707 0.707c-0.391 0.39-1.024 0.39-1.414 0s-0.391-1.024 0-1.414l0.707-0.707z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M8 4c-2.209 0-4 1.791-4 4s1.791 4 4 4c2.209 0 4-1.791 4-4s-1.791-4-4-4zM8 10.5c-1.381 0-2.5-1.119-2.5-2.5s1.119-2.5 2.5-2.5 2.5 1.119 2.5 2.5-1.119 2.5-2.5 2.5z"
        /> < title > { title } < / title > < / svg >
    }
}
