#[cfg(feature = "TbSwords")]
use leptos::*;
#[cfg(feature = "TbSwords")]
///This icon requires the feature `TbSwords` to be enabled.
#[component]
pub fn Swords(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-swords"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 3v5l-11 9l-4 4l-3 -3l4 -4l9 -11z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 13l6 6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14.32 17.32l3.68 3.68l3 -3l-3.365 -3.365" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M10 5.5l-2 -2.5h-5v5l3 2.5" /> <
        title > { title } < / title > < / svg >
    }
}
