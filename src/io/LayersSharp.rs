#[cfg(feature = "IoLayersSharp")]
use leptos::*;
#[cfg(feature = "IoLayersSharp")]
///This icon requires the feature `IoLayersSharp` to be enabled.
#[component]
pub fn LayersSharp(
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
        points = "480 150 256 48 32 150 256 254 480 150" />< polygon xmlns =
        "http://www.w3.org/2000/svg" points =
        "255.71 392.95 110.9 326.75 32 362 256 464 480 362 401.31 326.7 255.71 392.95"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M480,256l-75.53-33.53L256.1,290.6,107.33,222.43,32,256,256,358,480,256S480,256,480,256Z"
        /> < title > { title } < / title > < / svg >
    }
}
