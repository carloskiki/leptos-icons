#[cfg(feature = "IoSparklesOutline")]
use leptos::*;
#[cfg(feature = "IoSparklesOutline")]
///This icon requires the feature `IoSparklesOutline` to be enabled.
#[component]
pub fn SparklesOutline(
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
        stroke_witdh = "0" style = style id = "icons" viewBox = "0 0 512 512" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M259.92,262.91,216.4,149.77a9,9,0,0,0-16.8,0L156.08,262.91a9,9,0,0,1-5.17,5.17L37.77,311.6a9,9,0,0,0,0,16.8l113.14,43.52a9,9,0,0,1,5.17,5.17L199.6,490.23a9,9,0,0,0,16.8,0l43.52-113.14a9,9,0,0,1,5.17-5.17L378.23,328.4a9,9,0,0,0,0-16.8L265.09,268.08A9,9,0,0,1,259.92,262.91Z"
        fill = "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin =
        "round" stroke - width = "32" />< polygon xmlns = "http://www.w3.org/2000/svg"
        points = "108 68 88 16 68 68 16 88 68 108 88 160 108 108 160 88 108 68" fill =
        "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin = "round"
        stroke - width = "32" />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "426.67 117.33 400 48 373.33 117.33 304 144 373.33 170.67 400 240 426.67 170.67 496 144 426.67 117.33"
        fill = "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin =
        "round" stroke - width = "32" /> < title > { title } < / title > < / svg >
    }
}
