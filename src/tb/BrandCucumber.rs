#[cfg(feature = "TbBrandCucumber")]
use leptos::*;
#[cfg(feature = "TbBrandCucumber")]
///This icon requires the feature `TbBrandCucumber` to be enabled.
#[component]
pub fn BrandCucumber(
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
        "icon icon-tabler icon-tabler-brand-cucumber" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 10.99c-.01 5.52 -4.48 10 -10 10.01v-2.26l-.01 -.01c-4.28 -1.11 -6.86 -5.47 -5.76 -9.75a8 8 0 0 1 9.74 -5.76c3.53 .91 6.03 4.13 6.03 7.78v-.01z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M10.5 8l-.5 -1" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M13.5 14l.5 1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 12.5l-1 .5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 14l-.5 1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 8l.5 -1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 12.5l-1 -.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 10l-1 -.5" /> < title > { title } < / title
        > < / svg >
    }
}
