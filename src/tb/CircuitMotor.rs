#[cfg(feature = "TbCircuitMotor")]
use leptos::*;
#[cfg(feature = "TbCircuitMotor")]
///This icon requires the feature `TbCircuitMotor` to be enabled.
#[component]
pub fn CircuitMotor(
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
        "icon icon-tabler icon-tabler-circuit-motor" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12m-7 0a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 12h-3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 12h3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 14v-4l2 2l2 -2v4" /> < title > { title } <
        / title > < / svg >
    }
}
