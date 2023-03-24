#[cfg(feature = "FiPackage")]
use leptos::*;
#[cfg(feature = "FiPackage")]
///This icon requires the feature `FiPackage` to be enabled.
#[component]
pub fn Package(
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
        < line xmlns = "http://www.w3.org/2000/svg" x1 = "16.5" y1 = "9.4" x2 = "7.5" y2
        = "4.21" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"
        />< polyline xmlns = "http://www.w3.org/2000/svg" points =
        "3.27 6.96 12 12.01 20.73 6.96" />< line xmlns = "http://www.w3.org/2000/svg" x1
        = "12" y1 = "22.08" x2 = "12" y2 = "12" /> < title > { title } < / title > < /
        svg >
    }
}
