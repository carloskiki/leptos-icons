#[cfg(feature = "TbEggOff")]
use leptos::*;
#[cfg(feature = "TbEggOff")]
///This icon requires the feature `TbEggOff` to be enabled.
#[component]
pub fn EggOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-egg-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.927 17.934c-1.211 1.858 -3.351 2.953 -5.927 3.066c-4.2 0 -7 -2.763 -7 -6.917c0 -2.568 .753 -5.14 1.91 -7.158"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.642 4.628c1.034 -1.02 2.196 -1.63 3.358 -1.628c3.5 .007 7 5.545 7 11.083c0 .298 -.015 .587 -.045 .868"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > {
        title } < / title > < / svg >
    }
}
