#[cfg(feature = "TbMoodMinus")]
use leptos::*;
#[cfg(feature = "TbMoodMinus")]
///This icon requires the feature `TbMoodMinus` to be enabled.
#[component]
pub fn MoodMinus(
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
        "icon icon-tabler icon-tabler-mood-minus" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20.48 15.014a9 9 0 1 0 -7.956 5.97" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 10h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 10h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 19h6" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M9.5 15c.658 .64 1.56 1 2.5 1s1.842 -.36 2.5 -1" /> < title > { title } < /
        title > < / svg >
    }
}
