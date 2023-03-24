#[cfg(feature = "TbRegexOff")]
use leptos::*;
#[cfg(feature = "TbRegexOff")]
///This icon requires the feature `TbRegexOff` to be enabled.
#[component]
pub fn RegexOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-regex-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.5 15a2.5 2.5 0 1 1 0 5a2.5 2.5 0 0 1 0 -5z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 7.875l3 -1.687" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 7.875v3.375" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 7.875l-3 -1.687" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 7.875l3 1.688" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 4.5v3.375" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 7.875l-3 1.688" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
