#[cfg(feature = "TbMoodSearch")]
use leptos::*;
#[cfg(feature = "TbMoodSearch")]
///This icon requires the feature `TbMoodSearch` to be enabled.
#[component]
pub fn MoodSearch(
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
        "icon icon-tabler icon-tabler-mood-search" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 12a9 9 0 1 0 -8.99 9" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 10h.01" />< path xmlns = "http://www.w3.org/2000/svg" d = "M15 10h.01" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.5 15a3.556 3.556 0 0 0 1.823 .937c.221 .042 .448 .063 .677 .063" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M18 18m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20.2 20.2l1.8 1.8" /> < title > { title } < /
        title > < / svg >
    }
}
