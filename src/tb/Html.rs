#[cfg(feature = "TbHtml")]
use leptos::*;
#[cfg(feature = "TbHtml")]
///This icon requires the feature `TbHtml` to be enabled.
#[component]
pub fn Html(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-html"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M13 16v-8l2 5l2 -5v8" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M1 16v-8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 8v8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M1 12h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 8h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 8v8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 8v8h3" /> < title > { title } < / title > <
        / svg >
    }
}
