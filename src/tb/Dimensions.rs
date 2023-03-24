#[cfg(feature = "TbDimensions")]
use leptos::*;
#[cfg(feature = "TbDimensions")]
///This icon requires the feature `TbDimensions` to be enabled.
#[component]
pub fn Dimensions(
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
        "icon icon-tabler icon-tabler-dimensions" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 5h11" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 7l2 -2l-2 -2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 3l-2 2l2 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 10v11" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 19l2 2l2 -2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 12l-2 -2l-2 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M3 10m0 2a2 2 0 0 1 2 -2h7a2 2 0 0 1 2 2v7a2 2 0 0 1 -2 2h-7a2 2 0 0 1 -2 -2z"
        /> < title > { title } < / title > < / svg >
    }
}
