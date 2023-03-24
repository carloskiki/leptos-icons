#[cfg(feature = "TbUfo")]
use leptos::*;
#[cfg(feature = "TbUfo")]
///This icon requires the feature `TbUfo` to be enabled.
#[component]
pub fn Ufo(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-ufo" width
        = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.95 9.01c3.02 .739 5.05 2.123 5.05 3.714c0 2.367 -4.48 4.276 -10 4.276s-10 -1.909 -10 -4.276c0 -1.59 2.04 -2.985 5.07 -3.724"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 9c0 1.105 2.239 2 5 2s5 -.895 5 -2v-.035c0 -2.742 -2.239 -4.965 -5 -4.965s-5 2.223 -5 4.965v.035"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M15 17l2 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8.5 17l-1.5 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 14h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 13h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 13h.01" /> < title > { title } < / title >
        < / svg >
    }
}
