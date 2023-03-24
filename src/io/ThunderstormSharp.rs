#[cfg(feature = "IoThunderstormSharp")]
use leptos::*;
#[cfg(feature = "IoThunderstormSharp")]
///This icon requires the feature `IoThunderstormSharp` to be enabled.
#[component]
pub fn ThunderstormSharp(
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
        "M405.84,136.9A151.25,151.25,0,0,0,358.24,55a153,153,0,0,0-241.81,51.86C60.5,110.16,16,156.65,16,213.33,16,272.15,63.91,320,122.8,320h72.31L176,416h48v80L336,352H292.49l8-32H404.33a91.56,91.56,0,0,0,1.51-183.1Z"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "65.17" y = "360" width =
        "85.67" height = "32" transform = "translate(-276.6 304.44) rotate(-63.43)" /><
        rect xmlns = "http://www.w3.org/2000/svg" x = "94.11" y = "432" width = "67.78"
        height = "32" transform = "translate(-329.95 362.13) rotate(-63.43)" />< rect
        xmlns = "http://www.w3.org/2000/svg" x = "345.17" y = "360" width = "85.67"
        height = "32" transform = "translate(-121.83 554.88) rotate(-63.43)" />< rect
        xmlns = "http://www.w3.org/2000/svg" x = "374.11" y = "432" width = "67.78"
        height = "32" transform = "translate(-175.17 612.57) rotate(-63.43)" /> < title >
        { title } < / title > < / svg >
    }
}
