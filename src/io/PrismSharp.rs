#[cfg(feature = "IoPrismSharp")]
use leptos::*;
#[cfg(feature = "IoPrismSharp")]
///This icon requires the feature `IoPrismSharp` to be enabled.
#[component]
pub fn PrismSharp(
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
        = "M256,16,16,352,256,496,496,352Zm-20,96.82V437.35L73.73,340Z" /> < title > {
        title } < / title > < / svg >
    }
}
