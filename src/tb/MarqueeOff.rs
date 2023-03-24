#[cfg(feature = "TbMarqueeOff")]
use leptos::*;
#[cfg(feature = "TbMarqueeOff")]
///This icon requires the feature `TbMarqueeOff` to be enabled.
#[component]
pub fn MarqueeOff(
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
        "icon icon-tabler icon-tabler-marquee-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4 6c0 -.556 .227 -1.059 .593 -1.421" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 4h1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13.5 4h1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 4a2 2 0 0 1 2 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 9v1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 13.5v1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19.402 19.426a1.993 1.993 0 0 1 -1.402 .574"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M15 20h-1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10.5 20h-1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 20a2 2 0 0 1 -2 -2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 15v-1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 10.5v-1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
