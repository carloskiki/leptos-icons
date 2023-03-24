#[cfg(feature = "TbH2")]
use leptos::*;
#[cfg(feature = "TbH2")]
///This icon requires the feature `TbH2` to be enabled.
#[component]
pub fn H2(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-h-2" width
        = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 12a2 2 0 1 1 4 0c0 .591 -.417 1.318 -.816 1.858l-3.184 4.143l4 0" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M4 6v12" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 6v12" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 18h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 18h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 12h8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 6h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 6h2" /> < title > { title } < / title > < /
        svg >
    }
}
