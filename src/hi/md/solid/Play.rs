#[cfg(feature = "HiMdSolidPlay")]
use leptos::*;
#[cfg(feature = "HiMdSolidPlay")]
///This icon requires the feature `HiMdSolidPlay` to be enabled.
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M6.29995 2.84095C5.3011 2.21124 4 2.92906 4 4.10984V15.891C4 17.0718 5.3011 17.7896 6.29995 17.1599L15.6436 11.2693C16.577 10.6809 16.577 9.31997 15.6436 8.73152L6.29995 2.84095Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
