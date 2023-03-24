#[cfg(feature = "IoPrismOutline")]
use leptos::*;
#[cfg(feature = "IoPrismOutline")]
///This icon requires the feature `IoPrismOutline` to be enabled.
#[component]
pub fn PrismOutline(
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
        stroke_witdh = "0" style = style id = "icons" viewBox = "0 0 512 512" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M229.73,45.88,37.53,327.79a31.79,31.79,0,0,0,11.31,46L241,476.26a31.77,31.77,0,0,0,29.92,0l192.2-102.51a31.79,31.79,0,0,0,11.31-46L282.27,45.88A31.8,31.8,0,0,0,229.73,45.88Z"
        fill = "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin =
        "round" stroke - width = "32" />< line xmlns = "http://www.w3.org/2000/svg" x1 =
        "256" y1 = "32" x2 = "256" y2 = "480" fill = "none" stroke = "#000" stroke -
        linecap = "round" stroke - linejoin = "round" stroke - width = "32" /> < title >
        { title } < / title > < / svg >
    }
}
