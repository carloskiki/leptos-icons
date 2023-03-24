#[cfg(feature = "TbBuildingStore")]
use leptos::*;
#[cfg(feature = "TbBuildingStore")]
///This icon requires the feature `TbBuildingStore` to be enabled.
#[component]
pub fn BuildingStore(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-building-store" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 21l18 0" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 7v1a3 3 0 0 0 6 0v-1m0 1a3 3 0 0 0 6 0v-1m0 1a3 3 0 0 0 6 0v-1h-18l2 -4h14l2 4"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M5 21l0 -10.15" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M19 21l0 -10.15" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 21v-4a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v4" /> <
        title > { title } < / title > < / svg >
    }
}
