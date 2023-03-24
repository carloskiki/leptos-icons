#[cfg(feature = "OcSmMoveToTop")]
use leptos::*;
#[cfg(feature = "OcSmMoveToTop")]
///This icon requires the feature `OcSmMoveToTop` to be enabled.
#[component]
pub fn MoveToTop(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M8.53 1.22a.749.749 0 0 0-1.06 0L3.72 4.97a.749.749 0 1 0 1.06 1.06l2.47-2.469v6.689a.75.75 0 0 0 1.5 0V3.561l2.47 2.469a.749.749 0 1 0 1.06-1.06L8.53 1.22ZM3.75 13a.75.75 0 0 0 0 1.5h8.5a.75.75 0 0 0 0-1.5h-8.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
