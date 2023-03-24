#[cfg(feature = "FiTrash2")]
use leptos::*;
#[cfg(feature = "FiTrash2")]
///This icon requires the feature `FiTrash2` to be enabled.
#[component]
pub fn Trash2(
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
        < polyline xmlns = "http://www.w3.org/2000/svg" points = "3 6 5 6 21 6" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "10" y1 = "11" x2 = "10" y2 =
        "17" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "14" y1 = "11" x2 = "14"
        y2 = "17" /> < title > { title } < / title > < / svg >
    }
}
