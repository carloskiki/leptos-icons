#[cfg(feature = "TbIceCreamOff")]
use leptos::*;
#[cfg(feature = "TbIceCreamOff")]
///This icon requires the feature `TbIceCreamOff` to be enabled.
#[component]
pub fn IceCreamOff(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-ice-cream-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 21.5v-4.5" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M8 8v9h8v-1m0 -4v-5a4 4 0 0 0 -7.277 -2.294"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 10.5l1.74 -.76m2.79 -1.222l3.47 -1.518" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 14.5l4.488 -1.964" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
