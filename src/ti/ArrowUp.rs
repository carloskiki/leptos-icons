#[cfg(feature = "TiArrowUp")]
use leptos::*;
#[cfg(feature = "TiArrowUp")]
///This icon requires the feature `TiArrowUp` to be enabled.
#[component]
pub fn ArrowUp(
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
        "M13 5.586l-4.707 4.707c-.391.391-.391 1.023 0 1.414s1.023.391 1.414 0l2.293-2.293v7.586c0 .552.447 1 1 1s1-.448 1-1v-7.586l2.293 2.293c.195.195.451.293.707.293s.512-.098.707-.293c.391-.391.391-1.023 0-1.414l-4.707-4.707z"
        /> < title > { title } < / title > < / svg >
    }
}
