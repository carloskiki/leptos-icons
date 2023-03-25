#[cfg(feature = "TbCarTurbine")]
use leptos::*;
#[cfg(feature = "TbCarTurbine")]
///This icon requires the feature `TbCarTurbine` to be enabled.
#[component]
pub fn CarTurbine(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    let style = format!("{} color: {};", style, color);
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-car-turbine" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11 13m-4 0a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M18.86 11c.088 .66 .14 1.512 .14 2a8 8 0 1 1 -8 -8h6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 9c2.489 .108 4.489 .108 6 0" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M17 3m0 1a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M11 13l-3.5 -1.5" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M11 13l2.5 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8.5 16l2.5 -3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 13l3.5 -1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 9v4" /> < title > { title } < / title > < /
        svg >
    }
}
