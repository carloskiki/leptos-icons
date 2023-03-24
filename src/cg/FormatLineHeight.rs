#[cfg(feature = "CgFormatLineHeight")]
use leptos::*;
#[cfg(feature = "CgFormatLineHeight")]
///This icon requires the feature `CgFormatLineHeight` to be enabled.
#[component]
pub fn FormatLineHeight(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.09668 6.99707H7.17358L4.17358 3.99707L1.17358 6.99707H3.09668V17.0031H1.15881L4.15881 20.0031L7.15881 17.0031H5.09668V6.99707Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M22.8412 7H8.84119V5H22.8412V7Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M22.8412 11H8.84119V9H22.8412V11Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.84119 15H22.8412V13H8.84119V15Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M22.8412 19H8.84119V17H22.8412V19Z" fill =
        "currentColor" /> < title > { title } < / title > < / svg >
    }
}
