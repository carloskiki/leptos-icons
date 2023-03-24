#[cfg(feature = "FiTruck")]
use leptos::*;
#[cfg(feature = "FiTruck")]
///This icon requires the feature `FiTruck` to be enabled.
#[component]
pub fn Truck(
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
        < rect xmlns = "http://www.w3.org/2000/svg" x = "1" y = "3" width = "15" height =
        "13" />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "16 8 20 8 23 11 23 16 16 16 16 8" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "5.5" cy = "18.5" r = "2.5" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "18.5" cy = "18.5" r = "2.5" /> < title > {
        title } < / title > < / svg >
    }
}
