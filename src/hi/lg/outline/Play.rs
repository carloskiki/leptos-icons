#[cfg(feature = "HiLgOutlinePlay")]
use leptos::*;
#[cfg(feature = "HiLgOutlinePlay")]
///This icon requires the feature `HiLgOutlinePlay` to be enabled.
#[component]
pub fn Play(
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
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M5.25 5.65297C5.25 4.79729 6.1674 4.25486 6.91716 4.66723L18.4577 11.0145C19.2349 11.442 19.2349 12.5586 18.4577 12.986L6.91716 19.3334C6.1674 19.7457 5.25 19.2033 5.25 18.3476V5.65297Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
