#[cfg(feature = "OcSmFeedTag")]
use leptos::*;
#[cfg(feature = "OcSmFeedTag")]
///This icon requires the feature `OcSmFeedTag` to be enabled.
#[component]
pub fn FeedTag(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M7.22 6.5a.72.72 0 1 1-1.44 0 .72.72 0 0 1 1.44 0Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M8 16A8 8 0 1 1 8 0a8 8 0 0 1 0 16ZM4 5v3.38c.001.397.159.778.44 1.059l3.211 3.213a1.202 1.202 0 0 0 1.698 0l3.303-3.303a1.202 1.202 0 0 0 0-1.698L9.439 4.44A1.5 1.5 0 0 0 8.379 4H5a1 1 0 0 0-1 1Z"
        /> < title > { title } < / title > < / svg >
    }
}
