#[cfg(feature = "TiPowerOutline")]
use leptos::*;
#[cfg(feature = "TiPowerOutline")]
///This icon requires the feature `TiPowerOutline` to be enabled.
#[component]
pub fn PowerOutline(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = { size.clone() } height = { size
        } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.51 6.062c-.814-.815-1.98-1.05-3.01-.729v-.333c0-1.656-1.344-3-3-3s-3 1.344-3 3v.332c-1.029-.319-2.195-.085-3.01.73-1.605 1.606-2.49 3.741-2.49 6.011s.885 4.405 2.49 6.011c1.604 1.605 3.739 2.489 6.01 2.489s4.405-.884 6.01-2.489c1.605-1.605 2.49-3.74 2.49-6.011s-.885-4.405-2.49-6.011zm-7.01-1.062c0-.552.447-1 1-1s1 .448 1 1v5c0 .552-.447 1-1 1s-1-.448-1-1v-5zm-1 3.803v1.197c0 1.104.896 2 2 2s2-.896 2-2v-1.182c.095.284.248.554.475.78.661.661 1.025 1.54 1.025 2.475s-.364 1.814-1.025 2.476c-1.322 1.321-3.627 1.321-4.949 0-.662-.662-1.026-1.541-1.026-2.476s.364-1.814 1.025-2.476c.231-.23.383-.504.475-.794zm6.596 7.867c-1.228 1.228-2.859 1.903-4.596 1.903s-3.368-.676-4.596-1.903c-1.227-1.228-1.904-2.86-1.904-4.597s.677-3.369 1.904-4.597c.391-.391 1.023-.391 1.414 0s.391 1.023 0 1.414c-.85.851-1.318 1.981-1.318 3.183s.468 2.333 1.318 3.183c.85.85 1.979 1.317 3.182 1.317s2.332-.468 3.182-1.317c.851-.85 1.318-1.98 1.318-3.183s-.468-2.333-1.318-3.183c-.391-.391-.391-1.023 0-1.414s1.023-.391 1.414 0c1.227 1.229 1.904 2.861 1.904 4.597s-.677 3.369-1.904 4.597z"
        /> < title > { title } < / title > < / svg >
    }
}
