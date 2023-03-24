#[cfg(feature = "IoStarOutline")]
use leptos::*;
#[cfg(feature = "IoStarOutline")]
///This icon requires the feature `IoStarOutline` to be enabled.
#[component]
pub fn StarOutline(
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
        "M480,208H308L256,48,204,208H32l140,96L118,464,256,364,394,464,340,304Z" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /> < title > {
        title } < / title > < / svg >
    }
}
