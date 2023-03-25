#[cfg(feature = "ImHappy2")]
use leptos::*;
#[cfg(feature = "ImHappy2")]
///This icon requires the feature `ImHappy2` to be enabled.
#[component]
pub fn Happy2(
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
        "M8 0c-4.418 0-8 3.582-8 8s3.582 8 8 8 8-3.582 8-8-3.582-8-8-8zM11 4c0.552 0 1 0.672 1 1.5s-0.448 1.5-1 1.5-1-0.672-1-1.5 0.448-1.5 1-1.5zM5 4c0.552 0 1 0.672 1 1.5s-0.448 1.5-1 1.5-1-0.672-1-1.5 0.448-1.5 1-1.5zM8 14c-2.607 0-4.772-2.186-5-4.973 1.465 0.846 3.188 1.329 5 1.329s3.535-0.481 5-1.327c-0.228 2.788-2.393 4.971-5 4.971z"
        /> < title > { title } < / title > < / svg >
    }
}
