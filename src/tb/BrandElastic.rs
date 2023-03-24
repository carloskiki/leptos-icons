#[cfg(feature = "TbBrandElastic")]
use leptos::*;
#[cfg(feature = "TbBrandElastic")]
///This icon requires the feature `TbBrandElastic` to be enabled.
#[component]
pub fn BrandElastic(
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
        "icon icon-tabler icon-tabler-brand-elastic" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14 2a5 5 0 0 1 5 5c0 .712 -.232 1.387 -.5 2c1.894 .042 3.5 1.595 3.5 3.5c0 1.869 -1.656 3.4 -3.5 3.5c.333 .625 .5 1.125 .5 1.5a2.5 2.5 0 0 1 -2.5 2.5c-.787 0 -1.542 -.432 -2 -1c-.786 1.73 -2.476 3 -4.5 3a5 5 0 0 1 -4.583 -7a3.5 3.5 0 0 1 -.11 -6.992l.195 0a2.5 2.5 0 0 1 2 -4c.787 0 1.542 .432 2 1c.786 -1.73 2.476 -3 4.5 -3z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M8.5 9l-3 -1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9.5 5l-1 4l1 2l5 2l4 -4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18.499 16l-3 -.5l-1 -2.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14.5 19l1 -3.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5.417 15l4.083 -4" /> < title > { title } < /
        title > < / svg >
    }
}
