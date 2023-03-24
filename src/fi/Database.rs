#[cfg(feature = "FiDatabase")]
use leptos::*;
#[cfg(feature = "FiDatabase")]
///This icon requires the feature `FiDatabase` to be enabled.
#[component]
pub fn Database(
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
        < ellipse xmlns = "http://www.w3.org/2000/svg" cx = "12" cy = "5" rx = "9" ry =
        "3" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 12c0 1.66-4 3-9 3s-9-1.34-9-3" />< path xmlns = "http://www.w3.org/2000/svg"
        d = "M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5" /> < title > { title } < / title > < /
        svg >
    }
}
