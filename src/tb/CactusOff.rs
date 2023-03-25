#[cfg(feature = "TbCactusOff")]
use leptos::*;
#[cfg(feature = "TbCactusOff")]
///This icon requires the feature `TbCactusOff` to be enabled.
#[component]
pub fn CactusOff(
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
        "icon icon-tabler icon-tabler-cactus-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M6 9v1a3 3 0 0 0 3 3h1" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 8v5a3 3 0 0 1 -.129 .872m-2.014 2a3 3 0 0 1 -.857 .124h-1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 21v-11m0 -4v-1a2 2 0 1 1 4 0v5m0 4v7" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M7 21h10" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
