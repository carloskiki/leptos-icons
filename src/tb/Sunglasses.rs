#[cfg(feature = "TbSunglasses")]
use leptos::*;
#[cfg(feature = "TbSunglasses")]
///This icon requires the feature `TbSunglasses` to be enabled.
#[component]
pub fn Sunglasses(
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
        "icon icon-tabler icon-tabler-sunglasses" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 4h-2l-3 10" />< path xmlns = "http://www.w3.org/2000/svg" d = "M16 4h2l3 10"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M10 16h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 16.5a3.5 3.5 0 0 1 -7 0v-2.5h7v2.5" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 16.5a3.5 3.5 0 0 1 -7 0v-2.5h7v2.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 14l4.5 4.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 14l4.5 4.5" /> < title > { title } < /
        title > < / svg >
    }
}
