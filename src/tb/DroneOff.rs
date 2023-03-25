#[cfg(feature = "TbDroneOff")]
use leptos::*;
#[cfg(feature = "TbDroneOff")]
///This icon requires the feature `TbDroneOff` to be enabled.
#[component]
pub fn DroneOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-drone-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d = "M14 14h-4v-4"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M10 10l-3.5 -3.5" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M9.957 5.95a3.503 3.503 0 0 0 -2.917 -2.91m-3.02 .989a3.5 3.5 0 0 0 1.98 5.936"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M14 10l3.5 -3.5" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M18 9.965a3.5 3.5 0 1 0 -3.966 -3.965"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M14 14l3.5 3.5" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M14.035 18a3.5 3.5 0 0 0 5.936 1.98m.987 -3.026a3.503 3.503 0 0 0 -2.918 -2.913"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M10 14l-3.5 3.5" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M6 14.035a3.5 3.5 0 1 0 3.966 3.965"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > {
        title } < / title > < / svg >
    }
}
