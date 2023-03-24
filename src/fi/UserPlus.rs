#[cfg(feature = "FiUserPlus")]
use leptos::*;
#[cfg(feature = "FiUserPlus")]
///This icon requires the feature `FiUserPlus` to be enabled.
#[component]
pub fn UserPlus(
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
        "M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "8.5" cy = "7" r = "4" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "20" y1 = "8" x2 = "20" y2 = "14" />< line
        xmlns = "http://www.w3.org/2000/svg" x1 = "23" y1 = "11" x2 = "17" y2 = "11" /> <
        title > { title } < / title > < / svg >
    }
}
