#[cfg(feature = "TbMicroscopeOff")]
use leptos::*;
#[cfg(feature = "TbMicroscopeOff")]
///This icon requires the feature `TbMicroscopeOff` to be enabled.
#[component]
pub fn MicroscopeOff(
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
        "icon icon-tabler icon-tabler-microscope-off" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M5 21h14" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 18h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 18v3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 10l-1 1l3 3l1 -1m2 -2l3 -3l-3 -3l-3 3" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M10.5 12.5l-1.5 1.5" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M17 3l3 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 21a6 6 0 0 0 5.457 -3.505m.441 -3.599a6 6 0 0 0 -2.183 -3.608" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < /
        title > < / svg >
    }
}
