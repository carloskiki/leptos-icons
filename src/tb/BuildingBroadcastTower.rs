#[cfg(feature = "TbBuildingBroadcastTower")]
use leptos::*;
#[cfg(feature = "TbBuildingBroadcastTower")]
///This icon requires the feature `TbBuildingBroadcastTower` to be enabled.
#[component]
pub fn BuildingBroadcastTower(
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
        "icon icon-tabler icon-tabler-building-broadcast-tower" width = "24" height =
        "24" viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill =
        "none" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16.616 13.924a5 5 0 1 0 -9.23 0" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M20.307 15.469a9 9 0 1 0 -16.615 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M9 21l3 -9l3 9" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 19h4" /> < title > { title } < / title > <
        / svg >
    }
}
