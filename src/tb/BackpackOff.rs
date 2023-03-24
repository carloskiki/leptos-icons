#[cfg(feature = "TbBackpackOff")]
use leptos::*;
#[cfg(feature = "TbBackpackOff")]
///This icon requires the feature `TbBackpackOff` to be enabled.
#[component]
pub fn BackpackOff(
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
        "icon icon-tabler icon-tabler-backpack-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 6h3a6 6 0 0 1 6 6v3m-.129 3.872a3 3 0 0 1 -2.871 2.128h-8a3 3 0 0 1 -3 -3v-6a5.99 5.99 0 0 1 2.285 -4.712"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M10 6v-1a2 2 0 1 1 4 0v1" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 21v-4a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
