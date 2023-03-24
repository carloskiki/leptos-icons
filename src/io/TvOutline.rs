#[cfg(feature = "IoTvOutline")]
use leptos::*;
#[cfg(feature = "IoTvOutline")]
///This icon requires the feature `IoTvOutline` to be enabled.
#[component]
pub fn TvOutline(
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
        "http://www.w3.org/2000/svg" x = "32" y = "96" width = "448" height = "272" rx =
        "32.14" ry = "32.14" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "128" y1 = "416" x2 = "384" y2 = "416" style =
        "stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px" /> <
        title > { title } < / title > < / svg >
    }
}
