#[cfg(feature = "TbBrandTripadvisor")]
use leptos::*;
#[cfg(feature = "TbBrandTripadvisor")]
///This icon requires the feature `TbBrandTripadvisor` to be enabled.
#[component]
pub fn BrandTripadvisor(
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
        "icon icon-tabler icon-tabler-brand-tripadvisor" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.5 13.5m-1.5 0a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M17.5 13.5m-1.5 0a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17.5 9a4.5 4.5 0 1 0 3.5 1.671l1 -1.671h-4.5z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.5 9a4.5 4.5 0 1 1 -3.5 1.671l-1 -1.671h4.5z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10.5 15.5l1.5 2l1.5 -2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 6.75c2 -.667 4 -.667 6 0" /> < title > {
        title } < / title > < / svg >
    }
}
