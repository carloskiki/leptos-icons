#[cfg(feature = "TbApertureOff")]
use leptos::*;
#[cfg(feature = "TbApertureOff")]
///This icon requires the feature `TbApertureOff` to be enabled.
#[component]
pub fn ApertureOff(
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
        "icon icon-tabler icon-tabler-aperture-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.6 15h10.55" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.641 5.631a9 9 0 1 0 12.719 12.738m1.68 -2.318a9 9 0 0 0 -12.074 -12.098" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M7.395 7.534l2.416 7.438" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M17.032 4.636l-4.852 3.526m-2.334 1.695l-1.349 .98" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20.559 14.51l-8.535 -6.201" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12.257 20.916l2.123 -6.533m.984 -3.028l.154 -.473" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
