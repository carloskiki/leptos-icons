#[cfg(feature = "TbCherry")]
use leptos::*;
#[cfg(feature = "TbCherry")]
///This icon requires the feature `TbCherry` to be enabled.
#[component]
pub fn Cherry(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-cherry"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.5 16.5m-3.5 0a3.5 3.5 0 1 0 7 0a3.5 3.5 0 1 0 -7 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 18m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 13c.366 -2 1.866 -3.873 4.5 -5.6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 15c-1.333 -2.333 -2.333 -5.333 -1 -9" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 6c3.667 -2.667 7.333 -2.667 11 0c-3.667 2.667 -7.333 2.667 -11 0" /> < title
        > { title } < / title > < / svg >
    }
}
