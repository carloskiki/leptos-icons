#[cfg(feature = "TbPageBreak")]
use leptos::*;
#[cfg(feature = "TbPageBreak")]
///This icon requires the feature `TbPageBreak` to be enabled.
#[component]
pub fn PageBreak(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-page-break" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14 3v4a1 1 0 0 0 1 1h4" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 18v1a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 14h3m4.5 0h3m4.5 0h3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 10v-5a2 2 0 0 1 2 -2h7l5 5v2" /> < title > {
        title } < / title > < / svg >
    }
}
