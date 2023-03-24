#[cfg(feature = "FiCornerUpRight")]
use leptos::*;
#[cfg(feature = "FiCornerUpRight")]
///This icon requires the feature `FiCornerUpRight` to be enabled.
#[component]
pub fn CornerUpRight(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap =
        "round" stroke - linejoin = "round" width = { size.clone() } height = { size } >
        < polyline xmlns = "http://www.w3.org/2000/svg" points = "15 14 20 9 15 4" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M4 20v-7a4 4 0 0 1 4-4h12" /> <
        title > { title } < / title > < / svg >
    }
}
