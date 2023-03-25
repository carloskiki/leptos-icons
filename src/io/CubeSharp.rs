#[cfg(feature = "IoCubeSharp")]
use leptos::*;
#[cfg(feature = "IoCubeSharp")]
///This icon requires the feature `IoCubeSharp` to be enabled.
#[component]
pub fn CubeSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < polygon xmlns = "http://www.w3.org/2000/svg"
        points = "48 170 48 366.92 240 480 240 284 48 170" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M272,480,464,366.92V170L272,284ZM448,357.64h0Z"
        />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "448 144 256 32 64 144 256 256 448 144" /> < title > { title } < / title > < /
        svg >
    }
}
