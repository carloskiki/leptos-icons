#[cfg(feature = "TbCarCrane")]
use leptos::*;
#[cfg(feature = "TbCarCrane")]
///This icon requires the feature `TbCarCrane` to be enabled.
#[component]
pub fn CarCrane(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-car-crane"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 17m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 17m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 18h8m4 0h2v-6a5 5 0 0 0 -5 -5h-1l1.5 5h4.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 18v-11h3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 17v-5h9" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 12v-6l18 -3v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 12v-4l-4 -2" /> < title > { title } < /
        title > < / svg >
    }
}
