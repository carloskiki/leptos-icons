#[cfg(feature = "TbGenderAndrogyne")]
use leptos::*;
#[cfg(feature = "TbGenderAndrogyne")]
///This icon requires the feature `TbGenderAndrogyne` to be enabled.
#[component]
pub fn GenderAndrogyne(
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
        "icon icon-tabler icon-tabler-gender-androgyne" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13 11l6 -6" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 15m-5 0a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 9v-4h-4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16.5 10.5l-3 -3" /> < title > { title } < /
        title > < / svg >
    }
}
