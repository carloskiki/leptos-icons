#[cfg(feature = "ImCommand")]
use leptos::*;
#[cfg(feature = "ImCommand")]
///This icon requires the feature `ImCommand` to be enabled.
#[component]
pub fn Command(
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
        "M11.5 14c-1.379 0-2.5-1.121-2.5-2.5v-1.5h-2v1.5c0 1.379-1.122 2.5-2.5 2.5s-2.5-1.121-2.5-2.5 1.122-2.5 2.5-2.5h1.5v-2h-1.5c-1.378 0-2.5-1.122-2.5-2.5s1.122-2.5 2.5-2.5 2.5 1.122 2.5 2.5v1.5h2v-1.5c0-1.378 1.121-2.5 2.5-2.5s2.5 1.122 2.5 2.5-1.121 2.5-2.5 2.5h-1.5v2h1.5c1.379 0 2.5 1.121 2.5 2.5s-1.121 2.5-2.5 2.5zM10 10v1.5c0 0.827 0.673 1.5 1.5 1.5s1.5-0.673 1.5-1.5-0.673-1.5-1.5-1.5h-1.5zM4.5 10c-0.827 0-1.5 0.673-1.5 1.5s0.673 1.5 1.5 1.5 1.5-0.673 1.5-1.5v-1.5h-1.5zM7 9h2v-2h-2v2zM10 6h1.5c0.827 0 1.5-0.673 1.5-1.5s-0.673-1.5-1.5-1.5-1.5 0.673-1.5 1.5v1.5zM4.5 3c-0.827 0-1.5 0.673-1.5 1.5s0.673 1.5 1.5 1.5h1.5v-1.5c0-0.827-0.673-1.5-1.5-1.5z"
        /> < title > { title } < / title > < / svg >
    }
}
