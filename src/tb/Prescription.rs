#[cfg(feature = "TbPrescription")]
use leptos::*;
#[cfg(feature = "TbPrescription")]
///This icon requires the feature `TbPrescription` to be enabled.
#[component]
pub fn Prescription(
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
        "icon icon-tabler icon-tabler-prescription" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 19v-16h4.5a4.5 4.5 0 1 1 0 9h-4.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 21l-9 -9" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 21l6 -6" /> < title > { title } < / title >
        < / svg >
    }
}
