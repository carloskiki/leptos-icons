#[cfg(feature = "TbBrandCitymapper")]
use leptos::*;
#[cfg(feature = "TbBrandCitymapper")]
///This icon requires the feature `TbBrandCitymapper` to be enabled.
#[component]
pub fn BrandCitymapper(
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
        "icon icon-tabler icon-tabler-brand-citymapper" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 11a1 1 0 1 1 -1 1.013a1 1 0 0 1 1 -1v-.013z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M21 11a1 1 0 1 1 -1 1.013a1 1 0 0 1 1 -1v-.013z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 12h8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 9l3 3l-3 3" /> < title > { title } < /
        title > < / svg >
    }
}
