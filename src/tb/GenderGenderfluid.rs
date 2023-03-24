#[cfg(feature = "TbGenderGenderfluid")]
use leptos::*;
#[cfg(feature = "TbGenderGenderfluid")]
///This icon requires the feature `TbGenderGenderfluid` to be enabled.
#[component]
pub fn GenderGenderfluid(
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
        "icon icon-tabler icon-tabler-gender-genderfluid" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 15.464a4 4 0 1 0 4 -6.928a4 4 0 0 0 -4 6.928z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15.464 14l3 -5.196" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5.536 15.195l3 -5.196" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 12h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 9l-6 -6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5.5 8.5l3 -3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 21l-6 -6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 20l3 -3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 7v-4h4" /> < title > { title } < / title > <
        / svg >
    }
}
