#[cfg(feature = "IoPlanetOutline")]
use leptos::*;
#[cfg(feature = "IoPlanetOutline")]
///This icon requires the feature `IoPlanetOutline` to be enabled.
#[component]
pub fn PlanetOutline(
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
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M413.48,284.46c58.87,47.24,91.61,89,80.31,108.55-17.85,30.85-138.78-5.48-270.1-81.15S.37,149.84,18.21,119c11.16-19.28,62.58-12.32,131.64,14.09"
        style = "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< circle
        xmlns = "http://www.w3.org/2000/svg" cx = "256" cy = "256" r = "160" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /> < title > {
        title } < / title > < / svg >
    }
}
