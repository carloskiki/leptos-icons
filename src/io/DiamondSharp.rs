#[cfg(feature = "IoDiamondSharp")]
use leptos::*;
#[cfg(feature = "IoDiamondSharp")]
///This icon requires the feature `IoDiamondSharp` to be enabled.
#[component]
pub fn DiamondSharp(
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
        stroke_witdh = "0" style = style id = "icons" viewBox = "0 0 512 512" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < polygon xmlns
        = "http://www.w3.org/2000/svg" points =
        "396.31 32 264 32 348.19 144.26 396.31 32" />< polygon xmlns =
        "http://www.w3.org/2000/svg" points = "115.69 32 163.81 144.26 248 32 115.69 32"
        />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "256 74.67 192 160 320 160 256 74.67" />< polygon xmlns =
        "http://www.w3.org/2000/svg" points =
        "422.95 51.06 376.26 160 488 160 422.95 51.06" />< polygon xmlns =
        "http://www.w3.org/2000/svg" points = "89.05 51.06 23 160 135.74 160 89.05 51.06"
        />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "146.68 192 24 192 246.8 480 247.33 480 146.68 192" />< polygon xmlns =
        "http://www.w3.org/2000/svg" points =
        "365.32 192 264.67 480 265.2 480 488 192 365.32 192" />< polygon xmlns =
        "http://www.w3.org/2000/svg" points = "329.39 192 182.61 192 256 400 329.39 192"
        /> < title > { title } < / title > < / svg >
    }
}
