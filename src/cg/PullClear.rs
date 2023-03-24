#[cfg(feature = "CgPullClear")]
use leptos::*;
#[cfg(feature = "CgPullClear")]
///This icon requires the feature `CgPullClear` to be enabled.
#[component]
pub fn PullClear(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M4 6H2V16C2 17.1046 2.89543 18 4 18H20C21.1046 18 22 17.1046 22 16V6H20V16H4V6Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 12H18V14H6V12Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 8H6V10H18V8Z" fill = "currentColor" /> <
        title > { title } < / title > < / svg >
    }
}
