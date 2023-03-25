#[cfg(feature = "TbBuildingFortress")]
use leptos::*;
#[cfg(feature = "TbBuildingFortress")]
///This icon requires the feature `TbBuildingFortress` to be enabled.
#[component]
pub fn BuildingFortress(
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
        "icon icon-tabler icon-tabler-building-fortress" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 21h1a1 1 0 0 0 1 -1v-1h0a3 3 0 0 1 6 0m3 2h1a1 1 0 0 0 1 -1v-15l-3 -2l-3 2v6h-4v-6l-3 -2l-3 2v15a1 1 0 0 0 1 1h2m8 -2v1a1 1 0 0 0 1 1h2"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M7 7h0v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 10h0v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 13h0v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 7h0v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 10h0v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 13h0v.01" /> < title > { title } < / title
        > < / svg >
    }
}
