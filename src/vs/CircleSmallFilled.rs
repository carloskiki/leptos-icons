#[cfg(feature = "VsCircleSmallFilled")]
use leptos::*;
#[cfg(feature = "VsCircleSmallFilled")]
///This icon requires the feature `VsCircleSmallFilled` to be enabled.
#[component]
pub fn CircleSmallFilled(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 8a2 2 0 1 1-4 0 2 2 0 0 1 4 0z" /> < title
        > { title } < / title > < / svg >
    }
}
