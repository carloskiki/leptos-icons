#[cfg(feature = "FiAlertTriangle")]
use leptos::*;
#[cfg(feature = "FiAlertTriangle")]
///This icon requires the feature `FiAlertTriangle` to be enabled.
#[component]
pub fn AlertTriangle(
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
        < path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "12" y1 = "9" x2 = "12" y2 =
        "13" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "12" y1 = "17" x2 =
        "12.01" y2 = "17" /> < title > { title } < / title > < / svg >
    }
}
