#[cfg(feature = "IoLibraryOutline")]
use leptos::*;
#[cfg(feature = "IoLibraryOutline")]
///This icon requires the feature `IoLibraryOutline` to be enabled.
#[component]
pub fn LibraryOutline(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < rect xmlns =
        "http://www.w3.org/2000/svg" x = "32" y = "96" width = "64" height = "368" rx =
        "16" ry = "16" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "112" y1 = "224" x2 = "240" y2 = "224" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "112" y1 = "400" x2 = "240" y2
        = "400" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "112" y = "160" width = "128"
        height = "304" rx = "16" ry = "16" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "256" y = "48" width = "96" height = "416" rx =
        "16" ry = "16" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M422.46,96.11l-40.4,4.25c-11.12,1.17-19.18,11.57-17.93,23.1l34.92,321.59c1.26,11.53,11.37,20,22.49,18.84l40.4-4.25c11.12-1.17,19.18-11.57,17.93-23.1L445,115C443.69,103.42,433.58,94.94,422.46,96.11Z"
        style = "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /> <
        title > { title } < / title > < / svg >
    }
}
