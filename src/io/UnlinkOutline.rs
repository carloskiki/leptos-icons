#[cfg(feature = "IoUnlinkOutline")]
use leptos::*;
#[cfg(feature = "IoUnlinkOutline")]
///This icon requires the feature `IoUnlinkOutline` to be enabled.
#[component]
pub fn UnlinkOutline(
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
        = "M208,352H144a96,96,0,0,1,0-192h64" fill = "none" stroke = "#000" stroke -
        linecap = "round" stroke - linejoin = "round" stroke - width = "36" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M304,160h64a96,96,0,0,1,0,192H304" fill
        = "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin = "round"
        stroke - width = "36" /> < title > { title } < / title > < / svg >
    }
}
