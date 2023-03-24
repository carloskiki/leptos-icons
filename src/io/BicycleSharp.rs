#[cfg(feature = "IoBicycleSharp")]
use leptos::*;
#[cfg(feature = "IoBicycleSharp")]
///This icon requires the feature `IoBicycleSharp` to be enabled.
#[component]
pub fn BicycleSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M320,192l-29.5-60.1C284.32,118,284.32,118,264,118c-13.26,0-14.76,0-23,7.3l-71.7,69.1C161,202.85,160,203.85,160,221c0,12.67,3.78,14.61,18.51,22.9L240,278v90h32V254s-29-17-48.3-30l48.9-51.5c18.7,28.5,27.3,51.5,38,51.5H384V192Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M388,448a92,92,0,1,1,92-92A92.1,92.1,0,0,1,388,448Zm0-152a60,60,0,1,0,60,60A60.07,60.07,0,0,0,388,296Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M124,448a92,92,0,1,1,92-92A92.1,92.1,0,0,1,124,448Zm0-152a60,60,0,1,0,60,60A60.07,60.07,0,0,0,124,296Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M320,128a31.89,31.89,0,0,0,32-32.1A31.55,31.55,0,0,0,320.2,64a32,32,0,1,0-.2,64Z"
        /> < title > { title } < / title > < / svg >
    }
}
