#[cfg(feature = "ImSuperscript2")]
use leptos::*;
#[cfg(feature = "ImSuperscript2")]
///This icon requires the feature `ImSuperscript2` to be enabled.
#[component]
pub fn Superscript2(
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
        "M3.032 13l0.9-3h4.137l0.9 3h1.775l-3-10h-3.488l-3 10h1.776zM5.432 5h1.137l0.9 3h-2.937l0.9-3zM11 13l2.5-4 2.5 4h-5z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M13.5 2h-1c-0.276 0-0.5-0.224-0.5-0.5s0.224-0.5 0.5-0.5h2c0.276 0 0.5-0.224 0.5-0.5s-0.224-0.5-0.5-0.5h-2c-0.827 0-1.5 0.673-1.5 1.5 0 0.384 0.145 0.734 0.383 1 0.275 0.307 0.674 0.5 1.117 0.5h1c0.276 0 0.5 0.224 0.5 0.5s-0.224 0.5-0.5 0.5h-2c-0.276 0-0.5 0.224-0.5 0.5s0.224 0.5 0.5 0.5h2c0.827 0 1.5-0.673 1.5-1.5 0-0.384-0.145-0.734-0.383-1-0.275-0.307-0.674-0.5-1.117-0.5z"
        /> < title > { title } < / title > < / svg >
    }
}
