#[cfg(feature = "TbViewfinderOff")]
use leptos::*;
#[cfg(feature = "TbViewfinderOff")]
///This icon requires the feature `TbViewfinderOff` to be enabled.
#[component]
pub fn ViewfinderOff(
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
        "icon icon-tabler icon-tabler-viewfinder-off" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.65 5.623a9 9 0 1 0 12.71 12.745m1.684 -2.328a9 9 0 0 0 -12.094 -12.08" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M12 3v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 21v-3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 12h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 12h-3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 12v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
