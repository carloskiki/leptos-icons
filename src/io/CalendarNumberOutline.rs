#[cfg(feature = "IoCalendarNumberOutline")]
use leptos::*;
#[cfg(feature = "IoCalendarNumberOutline")]
///This icon requires the feature `IoCalendarNumberOutline` to be enabled.
#[component]
pub fn CalendarNumberOutline(
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
        stroke_witdh = "0" style = style id = "icons" viewBox = "0 0 512 512" width = {
        size.clone() } height = { size } > < rect xmlns = "http://www.w3.org/2000/svg" x
        = "48" y = "80" width = "416" height = "384" rx = "48" fill = "none" stroke =
        "#000" stroke - linejoin = "round" stroke - width = "32" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "128" y1 = "48" x2 = "128" y2 = "80" fill =
        "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin = "round"
        stroke - width = "32" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "384" y1
        = "48" x2 = "384" y2 = "80" fill = "none" stroke = "#000" stroke - linecap =
        "round" stroke - linejoin = "round" stroke - width = "32" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "464" y1 = "160" x2 = "48" y2 = "160" fill =
        "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin = "round"
        stroke - width = "32" />< polyline xmlns = "http://www.w3.org/2000/svg" points =
        "304 260 347.42 228 352 228 352 396" fill = "none" stroke = "#000" stroke -
        linecap = "round" stroke - linejoin = "round" stroke - width = "32" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M191.87,306.63c9.11,0,25.79-4.28,36.72-15.47a37.9,37.9,0,0,0,11.13-27.26c0-26.12-22.59-39.9-47.89-39.9-21.4,0-33.52,11.61-37.85,18.93"
        fill = "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin =
        "round" stroke - width = "32" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M149,374.16c4.88,8.27,19.71,25.84,43.88,25.84,28.59,0,52.12-15.94,52.12-43.82,0-12.62-3.66-24-11.58-32.07-12.36-12.64-31.25-17.48-41.55-17.48"
        fill = "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin =
        "round" stroke - width = "32" /> < title > { title } < / title > < / svg >
    }
}
