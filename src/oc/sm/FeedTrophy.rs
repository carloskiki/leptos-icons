#[cfg(feature = "OcSmFeedTrophy")]
use leptos::*;
#[cfg(feature = "OcSmFeedTrophy")]
///This icon requires the feature `OcSmFeedTrophy` to be enabled.
#[component]
pub fn FeedTrophy(
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
        "M11 5h1v1.146a1 1 0 0 1-.629.928L11 7.223V5ZM5 7.223l-.371-.149A1 1 0 0 1 4 6.146V5h1v2.223Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 16A8 8 0 1 1 8 0a8 8 0 0 1 0 16ZM3 5v1.146a2 2 0 0 0 1.257 1.858l.865.346a3.005 3.005 0 0 0 2.294 2.093C7.22 11.404 6.658 12 5.502 12H5.5a.5.5 0 0 0 0 1h5a.5.5 0 0 0 0-1c-1.158 0-1.72-.595-1.916-1.557a3.005 3.005 0 0 0 2.294-2.094l.865-.346A2 2 0 0 0 13 6.146V5a1 1 0 0 0-1-1H4a1 1 0 0 0-1 1Z"
        /> < title > { title } < / title > < / svg >
    }
}
