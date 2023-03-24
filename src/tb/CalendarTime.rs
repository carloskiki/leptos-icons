#[cfg(feature = "TbCalendarTime")]
use leptos::*;
#[cfg(feature = "TbCalendarTime")]
///This icon requires the feature `TbCalendarTime` to be enabled.
#[component]
pub fn CalendarTime(
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
        "icon icon-tabler icon-tabler-calendar-time" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.795 21h-6.795a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v4" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 18m-4 0a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 3v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 3v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 11h16" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 16.496v1.504l1 1" /> < title > { title } <
        / title > < / svg >
    }
}
