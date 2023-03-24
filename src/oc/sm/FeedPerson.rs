#[cfg(feature = "OcSmFeedPerson")]
use leptos::*;
#[cfg(feature = "OcSmFeedPerson")]
///This icon requires the feature `OcSmFeedPerson` to be enabled.
#[component]
pub fn FeedPerson(
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
        "M8 16A8 8 0 1 1 8 0a8 8 0 0 1 0 16Zm.847-8.145a2.502 2.502 0 1 0-1.694 0C5.471 8.261 4 9.775 4 11c0 .395.145.995 1 .995h6c.855 0 1-.6 1-.995 0-1.224-1.47-2.74-3.153-3.145Z"
        /> < title > { title } < / title > < / svg >
    }
}
