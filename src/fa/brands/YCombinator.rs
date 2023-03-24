#[cfg(feature = "FaBrandsYCombinator")]
use leptos::*;
#[cfg(feature = "FaBrandsYCombinator")]
///This icon requires the feature `FaBrandsYCombinator` to be enabled.
#[component]
pub fn YCombinator(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M448 32v448H0V32h448zM236 287.5L313.5 142h-32.7L235 233c-4.7 9.3-9 18.3-12.8 26.8L210 233l-45.2-91h-35l76.7 143.8v94.5H236v-92.8z"
        /> < title > { title } < / title > < / svg >
    }
}
