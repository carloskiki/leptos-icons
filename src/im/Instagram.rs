#[cfg(feature = "ImInstagram")]
use leptos::*;
#[cfg(feature = "ImInstagram")]
///This icon requires the feature `ImInstagram` to be enabled.
#[component]
pub fn Instagram(
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
        "M14.5 0h-13c-0.825 0-1.5 0.675-1.5 1.5v13c0 0.825 0.675 1.5 1.5 1.5h13c0.825 0 1.5-0.675 1.5-1.5v-13c0-0.825-0.675-1.5-1.5-1.5zM11 2.5c0-0.275 0.225-0.5 0.5-0.5h2c0.275 0 0.5 0.225 0.5 0.5v2c0 0.275-0.225 0.5-0.5 0.5h-2c-0.275 0-0.5-0.225-0.5-0.5v-2zM8 5c1.656 0 3 1.344 3 3s-1.344 3-3 3c-1.656 0-3-1.344-3-3s1.344-3 3-3zM14 13.5v0c0 0.275-0.225 0.5-0.5 0.5h-11c-0.275 0-0.5-0.225-0.5-0.5v0-6.5h1.1c-0.066 0.322-0.1 0.656-0.1 1 0 2.762 2.237 5 5 5s5-2.238 5-5c0-0.344-0.034-0.678-0.1-1h1.1v6.5z"
        /> < title > { title } < / title > < / svg >
    }
}
