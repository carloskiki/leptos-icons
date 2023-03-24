#[cfg(feature = "TbBugOff")]
use leptos::*;
#[cfg(feature = "TbBugOff")]
///This icon requires the feature `TbBugOff` to be enabled.
#[component]
pub fn BugOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-bug-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.884 5.873a3 3 0 0 1 5.116 2.127v1" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M13 9h3a6 6 0 0 1 1 3v1m-.298 3.705a5 5 0 0 1 -9.702 -1.705v-3a6 6 0 0 1 1 -3h1"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 13h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 13h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 20v-6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 19l3.35 -2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 7l3.75 2.4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 7l-3.75 2.4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
