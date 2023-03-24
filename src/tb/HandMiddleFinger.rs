#[cfg(feature = "TbHandMiddleFinger")]
use leptos::*;
#[cfg(feature = "TbHandMiddleFinger")]
///This icon requires the feature `TbHandMiddleFinger` to be enabled.
#[component]
pub fn HandMiddleFinger(
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
        "icon icon-tabler icon-tabler-hand-middle-finger" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 13v-2.5a1.5 1.5 0 0 1 3 0v1.5" />< path xmlns = "http://www.w3.org/2000/svg"
        d = "M14 10.5a1.5 1.5 0 0 1 3 0v1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M17 11.5a1.5 1.5 0 0 1 3 0v4.5a6 6 0 0 1 -6 6h-2h.208a6 6 0 0 1 -5.012 -2.7a69.74 69.74 0 0 1 -.196 -.3c-.312 -.479 -1.407 -2.388 -3.286 -5.728a1.5 1.5 0 0 1 .536 -2.022a1.867 1.867 0 0 1 2.28 .28l1.47 1.47"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11 11.5v-8a1.5 1.5 0 1 1 3 0v8.5" /> < title > { title } < / title > < / svg >
    }
}
