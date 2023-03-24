#[cfg(feature = "RiMapFillMotorbike")]
use leptos::*;
#[cfg(feature = "RiMapFillMotorbike")]
///This icon requires the feature `RiMapFillMotorbike` to be enabled.
#[component]
pub fn Motorbike(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path d =
        "M8.365 10L11.2 8H17v2h-5.144L9 12H2v-2h6.365zm.916 5.06l2.925-1.065.684 1.88-2.925 1.064a4.5 4.5 0 1 1-.684-1.88zM5.5 20a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5zm13 2a4.5 4.5 0 1 1 0-9 4.5 4.5 0 0 1 0 9zm0-2a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5zM4 11h6l2.6-1.733.28-1.046 1.932.518-1.922 7.131-1.822-.888.118-.44L9 16l-1-2H4v-3zm12.092-5H20v3h-2.816l1.92 5.276-1.88.684L15.056 9H15v-.152L13.6 5H11V3h4l1.092 3z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
