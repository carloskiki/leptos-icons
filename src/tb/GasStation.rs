#[cfg(feature = "TbGasStation")]
use leptos::*;
#[cfg(feature = "TbGasStation")]
///This icon requires the feature `TbGasStation` to be enabled.
#[component]
pub fn GasStation(
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
        "icon icon-tabler icon-tabler-gas-station" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14 11h1a2 2 0 0 1 2 2v3a1.5 1.5 0 0 0 3 0v-7l-3 -3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 20v-14a2 2 0 0 1 2 -2h6a2 2 0 0 1 2 2v14"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 20l12 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 7v1a1 1 0 0 0 1 1h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 11l10 0" /> < title > { title } < / title >
        < / svg >
    }
}
