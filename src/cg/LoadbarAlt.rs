#[cfg(feature = "CgLoadbarAlt")]
use leptos::*;
#[cfg(feature = "CgLoadbarAlt")]
///This icon requires the feature `CgLoadbarAlt` to be enabled.
#[component]
pub fn LoadbarAlt(
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
        fill = "none" width = { size.clone() } height = { size } > < rect xmlns =
        "http://www.w3.org/2000/svg" opacity = "0.3" x = "3" y = "10" width = "18" height
        = "4" rx = "2" fill = "currentColor" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "7" y = "10" width = "10" height = "4" rx = "2"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
