#[cfg(feature = "TbCarCrash")]
use leptos::*;
#[cfg(feature = "TbCarCrash")]
///This icon requires the feature `TbCarCrash` to be enabled.
#[component]
pub fn CarCrash(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-car-crash"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 17m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M7 6l4 5h1a2 2 0 0 1 2 2v4h-2m-4 0h-5m0 -6h8m-6 0v-5m2 0h-4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 8v-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 12h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17.5 15.5l1.5 1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17.5 8.5l1.5 -1.5" /> < title > { title } < /
        title > < / svg >
    }
}
