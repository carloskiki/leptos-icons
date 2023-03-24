#[cfg(feature = "TbMoodCheck")]
use leptos::*;
#[cfg(feature = "TbMoodCheck")]
///This icon requires the feature `TbMoodCheck` to be enabled.
#[component]
pub fn MoodCheck(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-mood-check" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20.925 13.163a8.998 8.998 0 0 0 -8.925 -10.163a9 9 0 0 0 0 18" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 10h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 10h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M9.5 15c.658 .64 1.56 1 2.5 1s1.842 -.36 2.5 -1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 19l2 2l4 -4" /> < title > { title } < /
        title > < / svg >
    }
}
