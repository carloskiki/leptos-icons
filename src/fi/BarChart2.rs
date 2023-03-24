#[cfg(feature = "FiBarChart2")]
use leptos::*;
#[cfg(feature = "FiBarChart2")]
///This icon requires the feature `FiBarChart2` to be enabled.
#[component]
pub fn BarChart2(
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
        < line xmlns = "http://www.w3.org/2000/svg" x1 = "18" y1 = "20" x2 = "18" y2 =
        "10" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "12" y1 = "20" x2 = "12"
        y2 = "4" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "6" y1 = "20" x2 =
        "6" y2 = "14" /> < title > { title } < / title > < / svg >
    }
}
