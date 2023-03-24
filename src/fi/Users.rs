#[cfg(feature = "FiUsers")]
use leptos::*;
#[cfg(feature = "FiUsers")]
///This icon requires the feature `FiUsers` to be enabled.
#[component]
pub fn Users(
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
        "M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "9" cy = "7" r = "4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M23 21v-2a4 4 0 0 0-3-3.87" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 3.13a4 4 0 0 1 0 7.75" /> < title > { title
        } < / title > < / svg >
    }
}
