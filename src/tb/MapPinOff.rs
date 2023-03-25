#[cfg(feature = "TbMapPinOff")]
use leptos::*;
#[cfg(feature = "TbMapPinOff")]
///This icon requires the feature `TbMapPinOff` to be enabled.
#[component]
pub fn MapPinOff(
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
        "icon icon-tabler icon-tabler-map-pin-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M9.44 9.435a3 3 0 0 0 4.126 4.124m1.434 -2.559a3 3 0 0 0 -3 -3" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M8.048 4.042a8 8 0 0 1 10.912 10.908m-1.8 2.206l-3.745 3.744a2 2 0 0 1 -2.827 0l-4.244 -4.243a8 8 0 0 1 -.48 -10.79"
        /> < title > { title } < / title > < / svg >
    }
}
