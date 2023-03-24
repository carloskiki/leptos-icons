#[cfg(feature = "ImWink")]
use leptos::*;
#[cfg(feature = "ImWink")]
///This icon requires the feature `ImWink` to be enabled.
#[component]
pub fn Wink(
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
        "M8 16c4.418 0 8-3.582 8-8s-3.582-8-8-8-8 3.582-8 8 3.582 8 8 8zM8 1.5c3.59 0 6.5 2.91 6.5 6.5s-2.91 6.5-6.5 6.5-6.5-2.91-6.5-6.5 2.91-6.5 6.5-6.5zM8.48 11.11c2.191-0.433 3.892-1.43 4.507-2.759-0.338 2.624-2.524 4.649-5.17 4.649-1.863 0-3.498-1.004-4.42-2.515 1.1 0.86 3.040 1.028 5.083 0.625zM10 5.5c0-0.828 0.448-1.5 1-1.5s1 0.672 1 1.5c0 0.828-0.448 1.5-1 1.5s-1-0.672-1-1.5zM5.5 5.805c-0.653 0-1.208 0.245-1.414 0.586-0.055-0.092-0.086-0.503-0.086-0.605 0-0.485 0.672-0.879 1.5-0.879s1.5 0.394 1.5 0.879c0 0.103-0.030 0.514-0.086 0.605-0.206-0.341-0.761-0.586-1.414-0.586z"
        /> < title > { title } < / title > < / svg >
    }
}
