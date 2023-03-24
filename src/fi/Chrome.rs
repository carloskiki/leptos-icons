#[cfg(feature = "FiChrome")]
use leptos::*;
#[cfg(feature = "FiChrome")]
///This icon requires the feature `FiChrome` to be enabled.
#[component]
pub fn Chrome(
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
        < circle xmlns = "http://www.w3.org/2000/svg" cx = "12" cy = "12" r = "10" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "12" cy = "12" r = "4" />< line
        xmlns = "http://www.w3.org/2000/svg" x1 = "21.17" y1 = "8" x2 = "12" y2 = "8" /><
        line xmlns = "http://www.w3.org/2000/svg" x1 = "3.95" y1 = "6.06" x2 = "8.54" y2
        = "14" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "10.88" y1 = "21.94" x2
        = "15.46" y2 = "14" /> < title > { title } < / title > < / svg >
    }
}
