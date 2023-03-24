#[cfg(feature = "TbTrees")]
use leptos::*;
#[cfg(feature = "TbTrees")]
///This icon requires the feature `TbTrees` to be enabled.
#[component]
pub fn Trees(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-trees"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16 5l3 3l-2 1l4 4l-3 1l4 4h-9" />< path xmlns = "http://www.w3.org/2000/svg" d
        = "M15 21l0 -3" />< path xmlns = "http://www.w3.org/2000/svg" d = "M8 13l-2 -2"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M8 12l2 -2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 21v-13" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M5.824 16a3 3 0 0 1 -2.743 -3.69a3 3 0 0 1 .304 -4.833a3 3 0 0 1 4.615 -3.707a3 3 0 0 1 4.614 3.707a3 3 0 0 1 .305 4.833a3 3 0 0 1 -2.919 3.695h-4z"
        /> < title > { title } < / title > < / svg >
    }
}
