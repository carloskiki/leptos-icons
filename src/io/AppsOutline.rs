#[cfg(feature = "IoAppsOutline")]
use leptos::*;
#[cfg(feature = "IoAppsOutline")]
///This icon requires the feature `IoAppsOutline` to be enabled.
#[component]
pub fn AppsOutline(
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
        "http://www.w3.org/2000/svg" x = "64" y = "64" width = "80" height = "80" rx =
        "40" ry = "40" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "216" y = "64" width = "80" height = "80" rx =
        "40" ry = "40" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "368" y = "64" width = "80" height = "80" rx =
        "40" ry = "40" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "64" y = "216" width = "80" height = "80" rx =
        "40" ry = "40" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "216" y = "216" width = "80" height = "80" rx =
        "40" ry = "40" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "368" y = "216" width = "80" height = "80" rx =
        "40" ry = "40" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "64" y = "368" width = "80" height = "80" rx =
        "40" ry = "40" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "216" y = "368" width = "80" height = "80" rx =
        "40" ry = "40" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "368" y = "368" width = "80" height = "80" rx =
        "40" ry = "40" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /> < title > {
        title } < / title > < / svg >
    }
}
