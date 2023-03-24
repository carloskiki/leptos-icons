#[cfg(feature = "TbBusStop")]
use leptos::*;
#[cfg(feature = "TbBusStop")]
///This icon requires the feature `TbBusStop` to be enabled.
#[component]
pub fn BusStop(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-bus-stop"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 3m0 1a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 17m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 5h7c2.761 0 5 3.134 5 7v5h-2" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M16 17h-8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 5l1.5 7h4.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9.5 10h7.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 5v5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 9v11" /> < title > { title } < / title > < /
        svg >
    }
}
