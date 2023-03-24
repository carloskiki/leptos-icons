#[cfg(feature = "TbBrandDeezer")]
use leptos::*;
#[cfg(feature = "TbBrandDeezer")]
///This icon requires the feature `TbBrandDeezer` to be enabled.
#[component]
pub fn BrandDeezer(
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
        "icon icon-tabler icon-tabler-brand-deezer" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 16.5h2v.5h-2z" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 16.5h2.5v.5h-2.5z" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16 17h-2.5v-.5h2.5z" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21.5 17h-2.5v-.5h2.5z" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21.5 13h-2.5v.5h2.5z" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21.5 9.5h-2.5v.5h2.5z" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21.5 6h-2.5v.5h2.5z" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16 13h-2.5v.5h2.5z" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 13.5h2.5v-.5h-2.5z" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 9.5h2.5v.5h-2.5z" /> < title > { title } < / title > < / svg >
    }
}
