#[cfg(feature = "FiPlusSquare")]
use leptos::*;
#[cfg(feature = "FiPlusSquare")]
///This icon requires the feature `FiPlusSquare` to be enabled.
#[component]
pub fn PlusSquare(
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
        < rect xmlns = "http://www.w3.org/2000/svg" x = "3" y = "3" width = "18" height =
        "18" rx = "2" ry = "2" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "12" y1
        = "8" x2 = "12" y2 = "16" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "8"
        y1 = "12" x2 = "16" y2 = "12" /> < title > { title } < / title > < / svg >
    }
}
