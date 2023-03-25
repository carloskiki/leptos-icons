#[cfg(feature = "ImTongue")]
use leptos::*;
#[cfg(feature = "ImTongue")]
///This icon requires the feature `ImTongue` to be enabled.
#[component]
pub fn Tongue(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M8 16c4.418 0 8-3.582 8-8s-3.582-8-8-8-8 3.582-8 8 3.582 8 8 8zM8 1.5c3.59 0 6.5 2.91 6.5 6.5s-2.91 6.5-6.5 6.5-6.5-2.91-6.5-6.5 2.91-6.5 6.5-6.5zM4 5c0-0.552 0.448-1 1-1s1 0.448 1 1c0 0.552-0.448 1-1 1s-1-0.448-1-1zM10 5c0-0.552 0.448-1 1-1s1 0.448 1 1c0 0.552-0.448 1-1 1s-1-0.448-1-1zM12 9v1h-1v1.5c0 0.828-0.672 1.5-1.5 1.5s-1.5-0.672-1.5-1.5v-1.5h-4v-1h8z"
        /> < title > { title } < / title > < / svg >
    }
}
