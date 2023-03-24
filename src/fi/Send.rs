#[cfg(feature = "FiSend")]
use leptos::*;
#[cfg(feature = "FiSend")]
///This icon requires the feature `FiSend` to be enabled.
#[component]
pub fn Send(
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
        < line xmlns = "http://www.w3.org/2000/svg" x1 = "22" y1 = "2" x2 = "11" y2 =
        "13" />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "22 2 15 22 11 13 2 9 22 2" /> < title > { title } < / title > < / svg >
    }
}
