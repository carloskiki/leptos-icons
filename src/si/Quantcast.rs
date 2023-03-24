#[cfg(feature = "SiQuantcast")]
use leptos::*;
#[cfg(feature = "SiQuantcast")]
///This icon requires the feature `SiQuantcast` to be enabled.
#[component]
pub fn Quantcast(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M5.791 11.1c0-3.574 1.92-7.18 6.21-7.18 4.288 0 6.208 3.606 6.208 7.18 0 1.55-.362 3.106-1.121 4.371l-4.906-.005 1.507 2.601A6.607 6.607 0 0 1 12 18.28c-4.29 0-6.209-3.606-6.209-7.18m9.96 10.53L17.124 24h4.911l-2.607-4.496c2.36-2.14 3.57-5.283 3.57-8.404C22.998 5.584 19.221 0 12 0 4.78 0 1.002 5.584 1.002 11.1c0 5.515 3.778 11.1 10.998 11.1 1.377 0 2.627-.205 3.75-.572Z"
        /> < title > { title } < / title > < / svg >
    }
}
