#[cfg(feature = "TbHealthRecognition")]
use leptos::*;
#[cfg(feature = "TbHealthRecognition")]
///This icon requires the feature `TbHealthRecognition` to be enabled.
#[component]
pub fn HealthRecognition(
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
        "icon icon-tabler icon-tabler-health-recognition" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4 8v-2a2 2 0 0 1 2 -2h2" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4 16v2a2 2 0 0 0 2 2h2" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16 4h2a2 2 0 0 1 2 2v2" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16 20h2a2 2 0 0 0 2 -2v-2" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.603 9.61a2.04 2.04 0 0 1 2.912 0l.485 .39l.5 -.396a2.035 2.035 0 0 1 2.897 .007a2.104 2.104 0 0 1 0 2.949l-3.397 3.44l-3.397 -3.44a2.104 2.104 0 0 1 0 -2.95z"
        /> < title > { title } < / title > < / svg >
    }
}
