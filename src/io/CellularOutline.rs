#[cfg(feature = "IoCellularOutline")]
use leptos::*;
#[cfg(feature = "IoCellularOutline")]
///This icon requires the feature `IoCellularOutline` to be enabled.
#[component]
pub fn CellularOutline(
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
        "http://www.w3.org/2000/svg" x = "416" y = "96" width = "64" height = "320" rx =
        "8" ry = "8" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "288" y = "176" width = "64" height = "240" rx =
        "8" ry = "8" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "160" y = "240" width = "64" height = "176" rx =
        "8" ry = "8" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "32" y = "304" width = "64" height = "112" rx =
        "8" ry = "8" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /> < title > {
        title } < / title > < / svg >
    }
}
