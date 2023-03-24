#[cfg(feature = "IoPinSharp")]
use leptos::*;
#[cfg(feature = "IoPinSharp")]
///This icon requires the feature `IoPinSharp` to be enabled.
#[component]
pub fn PinSharp(
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
        "M339,99a83,83,0,1,0-102,80.8V464l19,32,19-32V179.8A83.28,83.28,0,0,0,339,99Zm-59-6a21,21,0,1,1,21-21A21,21,0,0,1,280,93Z"
        /> < title > { title } < / title > < / svg >
    }
}
