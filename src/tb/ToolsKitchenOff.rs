#[cfg(feature = "TbToolsKitchenOff")]
use leptos::*;
#[cfg(feature = "TbToolsKitchenOff")]
///This icon requires the feature `TbToolsKitchenOff` to be enabled.
#[component]
pub fn ToolsKitchenOff(
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
        "icon icon-tabler icon-tabler-tools-kitchen-off" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 3h5l-.5 4.5m-.4 3.595l-.1 .905h-6l-.875 -7.874" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 18h2v3h-2z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M15.225 11.216c.42 -2.518 1.589 -5.177 4.775 -8.216v12h-1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 15v1m0 4v1h-1v-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 12v6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
