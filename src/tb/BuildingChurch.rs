#[cfg(feature = "TbBuildingChurch")]
use leptos::*;
#[cfg(feature = "TbBuildingChurch")]
///This icon requires the feature `TbBuildingChurch` to be enabled.
#[component]
pub fn BuildingChurch(
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
        "icon icon-tabler icon-tabler-building-church" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 21l18 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 21v-4a2 2 0 0 1 4 0v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 5l4 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 3l0 5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 21v-7m-2 2l8 -8l8 8m-2 -2v7" /> < title > {
        title } < / title > < / svg >
    }
}
