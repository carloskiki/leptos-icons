#[cfg(feature = "BiSolidShocked")]
use leptos::*;
#[cfg(feature = "BiSolidShocked")]
///This icon requires the feature `BiSolidShocked` to be enabled.
#[component]
pub fn Shocked(
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
        "M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm-5 8.5a1.5 1.5 0 1 1 3.001.001A1.5 1.5 0 0 1 7 10.5zm5 7.5c-1.657 0-3-1.119-3-2.5s1.343-2.5 3-2.5 3 1.119 3 2.5-1.343 2.5-3 2.5zm3.493-6.014a1.494 1.494 0 1 1 .001-2.987 1.494 1.494 0 0 1-.001 2.987z"
        /> < title > { title } < / title > < / svg >
    }
}
