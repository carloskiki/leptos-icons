#[cfg(feature = "TbLineHeight")]
use leptos::*;
#[cfg(feature = "TbLineHeight")]
///This icon requires the feature `TbLineHeight` to be enabled.
#[component]
pub fn LineHeight(
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
        "icon icon-tabler icon-tabler-line-height" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 8l3 -3l3 3" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 16l3 3l3 -3" />< path xmlns = "http://www.w3.org/2000/svg" d = "M6 5l0 14"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M13 6l7 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 12l7 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 18l7 0" /> < title > { title } < / title >
        < / svg >
    }
}
