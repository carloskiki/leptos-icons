#[cfg(feature = "TbTopologyStar3")]
use leptos::*;
#[cfg(feature = "TbTopologyStar3")]
///This icon requires the feature `TbTopologyStar3` to be enabled.
#[component]
pub fn TopologyStar3(
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
        "icon icon-tabler icon-tabler-topology-star-3" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 19a2 2 0 1 0 -4 0a2 2 0 0 0 4 0z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 5a2 2 0 1 0 -4 0a2 2 0 0 0 4 0z" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M10 5a2 2 0 1 0 -4 0a2 2 0 0 0 4 0z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 19a2 2 0 1 0 -4 0a2 2 0 0 0 4 0z" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M14 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M22 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 12h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 12h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 7l-2 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 7l2 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 14l-2 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 14l2 3" /> < title > { title } < / title >
        < / svg >
    }
}
