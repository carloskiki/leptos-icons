#[cfg(feature = "FiScissors")]
use leptos::*;
#[cfg(feature = "FiScissors")]
///This icon requires the feature `FiScissors` to be enabled.
#[component]
pub fn Scissors(
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
        < circle xmlns = "http://www.w3.org/2000/svg" cx = "6" cy = "6" r = "3" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "6" cy = "18" r = "3" />< line
        xmlns = "http://www.w3.org/2000/svg" x1 = "20" y1 = "4" x2 = "8.12" y2 = "15.88"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "14.47" y1 = "14.48" x2 = "20"
        y2 = "20" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "8.12" y1 = "8.12"
        x2 = "12" y2 = "12" /> < title > { title } < / title > < / svg >
    }
}
