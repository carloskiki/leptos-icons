#[cfg(feature = "TbGlassOff")]
use leptos::*;
#[cfg(feature = "TbGlassOff")]
///This icon requires the feature `TbGlassOff` to be enabled.
#[component]
pub fn GlassOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-glass-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d = "M8 21l8 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M12 15l0 6" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M7 3h10l1 7a4.511 4.511 0 0 1 -1.053 2.94m-2.386 1.625a7.48 7.48 0 0 1 -2.561 .435c-3.314 0 -6 -1.988 -6 -5l.5 -3.495"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > {
        title } < / title > < / svg >
    }
}
