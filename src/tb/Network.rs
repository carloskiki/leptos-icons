#[cfg(feature = "TbNetwork")]
use leptos::*;
#[cfg(feature = "TbNetwork")]
///This icon requires the feature `TbNetwork` to be enabled.
#[component]
pub fn Network(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-network"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 9m-6 0a6 6 0 1 0 12 0a6 6 0 1 0 -12 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 3c1.333 .333 2 2.333 2 6s-.667 5.667 -2 6"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 3c-1.333 .333 -2 2.333 -2 6s.667 5.667 2 6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 9h12" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 19h7" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 19h7" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 19m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M12 15v2" /> < title > { title } <
        / title > < / svg >
    }
}
