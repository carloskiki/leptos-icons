#[cfg(feature = "TbLegoOff")]
use leptos::*;
#[cfg(feature = "TbLegoOff")]
///This icon requires the feature `TbLegoOff` to be enabled.
#[component]
pub fn LegoOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-lego-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d = "M9.5 11h.01" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M9.5 15a3.5 3.5 0 0 0 5 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 4v-1h8v2h1a3 3 0 0 1 3 3v8m-.884 3.127a2.99 2.99 0 0 1 -2.116 .873v1h-10v-1a3 3 0 0 1 -3 -3v-9c0 -1.083 .574 -2.032 1.435 -2.56"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > {
        title } < / title > < / svg >
    }
}
