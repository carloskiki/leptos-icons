#[cfg(feature = "FaBrandsCreativeCommonsSa")]
use leptos::*;
#[cfg(feature = "FaBrandsCreativeCommonsSa")]
///This icon requires the feature `FaBrandsCreativeCommonsSa` to be enabled.
#[component]
pub fn CreativeCommonsSa(
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
        stroke_witdh = "0" style = style viewBox = "0 0 496 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M247.6 8C389.4 8 496 118.1 496 256c0 147.1-118.5 248-248.4 248C113.6 504 0 394.5 0 256 0 123.1 104.7 8 247.6 8zm.8 44.7C130.2 52.7 44.7 150.6 44.7 256c0 109.8 91.2 202.8 203.7 202.8 103.2 0 202.8-81.1 202.8-202.8.1-113.8-90.2-203.3-202.8-203.3zM137.7 221c13-83.9 80.5-95.7 108.9-95.7 99.8 0 127.5 82.5 127.5 134.2 0 63.6-41 132.9-128.9 132.9-38.9 0-99.1-20-109.4-97h62.5c1.5 30.1 19.6 45.2 54.5 45.2 23.3 0 58-18.2 58-82.8 0-82.5-49.1-80.6-56.7-80.6-33.1 0-51.7 14.6-55.8 43.8h18.2l-49.2 49.2-49-49.2h19.4z"
        /> < title > { title } < / title > < / svg >
    }
}
