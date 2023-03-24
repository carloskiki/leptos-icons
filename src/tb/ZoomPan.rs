#[cfg(feature = "TbZoomPan")]
use leptos::*;
#[cfg(feature = "TbZoomPan")]
///This icon requires the feature `TbZoomPan` to be enabled.
#[component]
pub fn ZoomPan(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-zoom-pan"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 17l-2.5 -2.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 5l2 -2l2 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 10l2 2l-2 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 10l-2 2l2 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 19l2 2l2 -2" /> < title > { title } < /
        title > < / svg >
    }
}
