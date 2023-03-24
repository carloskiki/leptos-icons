#[cfg(feature = "FiAnchor")]
use leptos::*;
#[cfg(feature = "FiAnchor")]
///This icon requires the feature `FiAnchor` to be enabled.
#[component]
pub fn Anchor(
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
        < circle xmlns = "http://www.w3.org/2000/svg" cx = "12" cy = "5" r = "3" />< line
        xmlns = "http://www.w3.org/2000/svg" x1 = "12" y1 = "22" x2 = "12" y2 = "8" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M5 12H2a10 10 0 0 0 20 0h-3" /> <
        title > { title } < / title > < / svg >
    }
}
