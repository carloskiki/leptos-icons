#[cfg(feature = "IoFilmOutline")]
use leptos::*;
#[cfg(feature = "IoFilmOutline")]
///This icon requires the feature `IoFilmOutline` to be enabled.
#[component]
pub fn FilmOutline(
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
        "http://www.w3.org/2000/svg" x = "48" y = "96" width = "416" height = "320" rx =
        "28" ry = "28" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "384" y = "336" width = "80" height = "80" rx =
        "28" ry = "28" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "384" y = "256" width = "80" height = "80" rx =
        "28" ry = "28" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "384" y = "176" width = "80" height = "80" rx =
        "28" ry = "28" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "384" y = "96" width = "80" height = "80" rx =
        "28" ry = "28" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "48" y = "336" width = "80" height = "80" rx =
        "28" ry = "28" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "48" y = "256" width = "80" height = "80" rx =
        "28" ry = "28" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "48" y = "176" width = "80" height = "80" rx =
        "28" ry = "28" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "48" y = "96" width = "80" height = "80" rx =
        "28" ry = "28" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "128" y = "96" width = "256" height = "160" rx =
        "28" ry = "28" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "128" y = "256" width = "256" height = "160" rx
        = "28" ry = "28" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /> < title > {
        title } < / title > < / svg >
    }
}
