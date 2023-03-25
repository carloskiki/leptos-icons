#[cfg(feature = "TbPlugConnectedX")]
use leptos::*;
#[cfg(feature = "TbPlugConnectedX")]
///This icon requires the feature `TbPlugConnectedX` to be enabled.
#[component]
pub fn PlugConnectedX(
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
        "icon icon-tabler icon-tabler-plug-connected-x" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M20 16l-4 4" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M7 12l5 5l-1.5 1.5a3.536 3.536 0 1 1 -5 -5l1.5 -1.5z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M17 12l-5 -5l1.5 -1.5a3.536 3.536 0 1 1 5 5l-1.5 1.5z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 21l2.5 -2.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18.5 5.5l2.5 -2.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 11l-2 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 14l-2 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 16l4 4" /> < title > { title } < / title >
        < / svg >
    }
}
