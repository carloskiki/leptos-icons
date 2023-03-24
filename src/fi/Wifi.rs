#[cfg(feature = "FiWifi")]
use leptos::*;
#[cfg(feature = "FiWifi")]
///This icon requires the feature `FiWifi` to be enabled.
#[component]
pub fn Wifi(
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
        < path xmlns = "http://www.w3.org/2000/svg" d = "M5 12.55a11 11 0 0 1 14.08 0"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M1.42 9a16 16 0 0 1 21.16 0"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M8.53 16.11a6 6 0 0 1 6.95 0"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "12" y1 = "20" x2 = "12.01" y2
        = "20" /> < title > { title } < / title > < / svg >
    }
}
