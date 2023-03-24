#[cfg(feature = "OcSmFeedHeart")]
use leptos::*;
#[cfg(feature = "OcSmFeedHeart")]
///This icon requires the feature `OcSmFeedHeart` to be enabled.
#[component]
pub fn FeedHeart(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M8 16A8 8 0 1 1 8 0a8 8 0 0 1 0 16Zm2.33-11.5c-1.22 0-1.83.5-2.323 1.136C7.513 5 6.903 4.5 5.682 4.5c-1.028 0-2.169.784-2.169 2.5 0 1.499 1.493 3.433 3.246 4.517.52.321.89.479 1.248.484.357-.005.728-.163 1.247-.484C11.007 10.433 12.5 8.5 12.5 7c0-1.716-1.14-2.5-2.17-2.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
