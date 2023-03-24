#[cfg(feature = "BiLogosVuejs")]
use leptos::*;
#[cfg(feature = "BiLogosVuejs")]
///This icon requires the feature `BiLogosVuejs` to be enabled.
#[component]
pub fn Vuejs(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "m12 12.765 5.592-9.437h-3.276L12 7.33v.002L9.688 3.328h-3.28z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M18.461 3.332 12 14.235 5.539 3.332H1.992L12 20.672l10.008-17.34z" /> < title >
        { title } < / title > < / svg >
    }
}
