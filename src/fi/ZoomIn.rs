#[cfg(feature = "FiZoomIn")]
use leptos::*;
#[cfg(feature = "FiZoomIn")]
///This icon requires the feature `FiZoomIn` to be enabled.
#[component]
pub fn ZoomIn(
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
        < circle xmlns = "http://www.w3.org/2000/svg" cx = "11" cy = "11" r = "8" /><
        line xmlns = "http://www.w3.org/2000/svg" x1 = "21" y1 = "21" x2 = "16.65" y2 =
        "16.65" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "11" y1 = "8" x2 =
        "11" y2 = "14" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "8" y1 = "11"
        x2 = "14" y2 = "11" /> < title > { title } < / title > < / svg >
    }
}
