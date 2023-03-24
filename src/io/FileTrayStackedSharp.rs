#[cfg(feature = "IoFileTrayStackedSharp")]
use leptos::*;
#[cfg(feature = "IoFileTrayStackedSharp")]
///This icon requires the feature `IoFileTrayStackedSharp` to be enabled.
#[component]
pub fn FileTrayStackedSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M448,16H64L32,176V320H480V176ZM436,176H320a64,64,0,0,1-128,0H76L98,58H414Z" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M320,352a64,64,0,0,1-128,0H32V496H480V352Z" /> < title > { title } < / title > <
        / svg >
    }
}
