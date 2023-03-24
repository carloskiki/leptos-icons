#[cfg(feature = "TbLollipop")]
use leptos::*;
#[cfg(feature = "TbLollipop")]
///This icon requires the feature `TbLollipop` to be enabled.
#[component]
pub fn Lollipop(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-lollipop"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14 10m-7 0a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 10a3.5 3.5 0 0 0 -7 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 10a3.5 3.5 0 0 1 -7 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 17a3.5 3.5 0 0 0 0 -7" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 3a3.5 3.5 0 0 0 0 7" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 21l6 -6" /> < title > { title } < / title >
        < / svg >
    }
}
