#[cfg(feature = "TbRadio")]
use leptos::*;
#[cfg(feature = "TbRadio")]
///This icon requires the feature `TbRadio` to be enabled.
#[component]
pub fn Radio(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-radio"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14 3l-9.371 3.749a1 1 0 0 0 -.629 .928v11.323a1 1 0 0 0 1 1h14a1 1 0 0 0 1 -1v-11a1 1 0 0 0 -1 -1h-14.5"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M4 12h16" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 12v-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 16v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 16v.01" /> < title > { title } < / title >
        < / svg >
    }
}
