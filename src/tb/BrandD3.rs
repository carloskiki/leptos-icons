#[cfg(feature = "TbBrandD3")]
use leptos::*;
#[cfg(feature = "TbBrandD3")]
///This icon requires the feature `TbBrandD3` to be enabled.
#[component]
pub fn BrandD3(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-brand-d3"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 4h1.8c3.976 0 7.2 3.582 7.2 8s-3.224 8 -7.2 8h-1.8" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 4h5.472c1.948 0 3.528 1.79 3.528 4s-1.58 4 -3.528 4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17.472 12h-2.472" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17.472 12h-2.352" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M17.472 12c1.948 0 3.528 1.79 3.528 4s-1.58 4 -3.528 4h-5.472" /> < title > {
        title } < / title > < / svg >
    }
}
