#[cfg(feature = "TbZoomReplace")]
use leptos::*;
#[cfg(feature = "TbZoomReplace")]
///This icon requires the feature `TbZoomReplace` to be enabled.
#[component]
pub fn ZoomReplace(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-zoom-replace" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 21l-6 -6" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.291 8a7 7 0 0 1 5.077 -4.806a7.021 7.021 0 0 1 8.242 4.403" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 4v4h-4" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M16.705 12a7 7 0 0 1 -5.074 4.798a7.021 7.021 0 0 1 -8.241 -4.403" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M3 16v-4h4" /> < title > { title } < /
        title > < / svg >
    }
}
