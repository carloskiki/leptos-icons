#[cfg(feature = "BiRegularSelection")]
use leptos::*;
#[cfg(feature = "BiRegularSelection")]
///This icon requires the feature `BiRegularSelection` to be enabled.
#[component]
pub fn Selection(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M20 9a1 1 0 0 0 1-1V4a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v1H9V4a1 1 0 0 0-1-1H4a1 1 0 0 0-1 1v4a1 1 0 0 0 1 1h1v6H4a1 1 0 0 0-1 1v4a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1v-1h6v1a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1v-4a1 1 0 0 0-1-1h-1V9h1zm-3-4h2v2h-2V5zM5 5h2v2H5V5zm2 14H5v-2h2v2zm12 0h-2v-2h2v2zm-2-4h-1a1 1 0 0 0-1 1v1H9v-1a1 1 0 0 0-1-1H7V9h1a1 1 0 0 0 1-1V7h6v1a1 1 0 0 0 1 1h1v6z"
        /> < title > { title } < / title > < / svg >
    }
}
