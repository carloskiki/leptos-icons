#[cfg(feature = "TbSpider")]
use leptos::*;
#[cfg(feature = "TbSpider")]
///This icon requires the feature `TbSpider` to be enabled.
#[component]
pub fn Spider(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-spider"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M5 4v2l5 5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M2.5 9.5l1.5 1.5h6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 19v-2l6 -6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 4v2l-5 5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21.5 9.5l-1.5 1.5h-6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 19v-2l-6 -6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 15m-4 0a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 9m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> < title > { title } < / title > < /
        svg >
    }
}
