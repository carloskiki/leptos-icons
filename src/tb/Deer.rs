#[cfg(feature = "TbDeer")]
use leptos::*;
#[cfg(feature = "TbDeer")]
///This icon requires the feature `TbDeer` to be enabled.
#[component]
pub fn Deer(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-deer"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 3c0 2 1 3 4 3c2 0 3 1 3 3"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 3c0 2 -1 3 -4 3c-2 0 -3 .333 -3 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 18c-1 0 -4 -3 -4 -6c0 -2 1.333 -3 4 -3s4 1 4 3c0 3 -3 6 -4 6" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M15.185 14.889l.095 -.18a4 4 0 1 1 -6.56 0"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 3c0 1.333 -.333 2.333 -1 3" />< path xmlns = "http://www.w3.org/2000/svg" d
        = "M7 3c0 1.333 .333 2.333 1 3" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 6c-2.667 .667 -4.333 1.667 -5 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 6c2.667 .667 4.333 1.667 5 3" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M8.5 10l-1.5 -1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15.5 10l1.5 -1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 15h.01" /> < title > { title } < / title >
        < / svg >
    }
}
