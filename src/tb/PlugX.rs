#[cfg(feature = "TbPlugX")]
use leptos::*;
#[cfg(feature = "TbPlugX")]
///This icon requires the feature `TbPlugX` to be enabled.
#[component]
pub fn PlugX(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-plug-x"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13.55 17.733a5.806 5.806 0 0 1 -7.356 -4.052a5.81 5.81 0 0 1 1.537 -5.627l2.054 -2.054l7.165 7.165"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M4 20l3.5 -3.5" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M15 4l-3.5 3.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 9l-3.5 3.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 16l4 4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 16l-4 4" /> < title > { title } < / title >
        < / svg >
    }
}
