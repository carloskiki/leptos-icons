#[cfg(feature = "SiByjus")]
use leptos::*;
#[cfg(feature = "SiByjus")]
///This icon requires the feature `SiByjus` to be enabled.
#[component]
pub fn Byjus(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M2.327.016A2.325 2.325 0 0 0 0 2.34v19.32a2.325 2.325 0 0 0 2.327 2.323h19.346A2.325 2.325 0 0 0 24 21.66V2.34A2.325 2.325 0 0 0 21.673.016zm10.054 3.496a3.443 3.443 0 0 1 .07 0 4.317 4.317 0 0 1 3.267 1.462 4.447 4.447 0 0 1 .961 2.365 4.157 4.157 0 0 1-.456 2.27 5.024 5.024 0 0 1 2.424 2.008 5.237 5.237 0 0 1 .73 3.374 4.68 4.68 0 0 1-1.15 2.466 4.84 4.84 0 0 1-2.26 1.535l-4.987 1.439a1.494 1.494 0 0 1-.41.058 1.497 1.497 0 0 1-1.432-1.075L5.524 6.909a1.487 1.487 0 0 1 1.018-1.841l4.956-1.429a3.443 3.443 0 0 1 .883-.127zm.248.861a3.091 3.091 0 0 0-.855.122L6.94 5.888a.744.744 0 0 0-.51.922l3.53 12.206a.745.745 0 0 0 .921.509l4.664-1.345a4.085 4.085 0 0 0-.896-8.003 3.297 3.297 0 0 0 1.138-2.272 3.479 3.479 0 0 0-.928-2.549 2.989 2.989 0 0 0-2.23-.983Z"
        /> < title > { title } < / title > < / svg >
    }
}
