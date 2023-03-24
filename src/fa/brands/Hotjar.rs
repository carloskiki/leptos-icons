#[cfg(feature = "FaBrandsHotjar")]
use leptos::*;
#[cfg(feature = "FaBrandsHotjar")]
///This icon requires the feature `FaBrandsHotjar` to be enabled.
#[component]
pub fn Hotjar(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M414.9 161.5C340.2 29 121.1 0 121.1 0S222.2 110.4 93 197.7C11.3 252.8-21 324.4 14 402.6c26.8 59.9 83.5 84.3 144.6 93.4-29.2-55.1-6.6-122.4-4.1-129.6 57.1 86.4 165 0 110.8-93.9 71 15.4 81.6 138.6 27.1 215.5 80.5-25.3 134.1-88.9 148.8-145.6 15.5-59.3 3.7-127.9-26.3-180.9z"
        /> < title > { title } < / title > < / svg >
    }
}
