#[cfg(feature = "CgTrophy")]
use leptos::*;
#[cfg(feature = "CgTrophy")]
///This icon requires the feature `CgTrophy` to be enabled.
#[component]
pub fn Trophy(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M13 15.9C15.2822 15.4367 17 13.419 17 11V4H7V11C7 13.419 8.71776 15.4367 11 15.9V18H9V20H15V18H13V15.9ZM9 6H15V11C15 12.6569 13.6569 14 12 14C10.3431 14 9 12.6569 9 11V6Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 6H20V11H18V6Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 6H4V11H6V6Z" fill = "currentColor" /> <
        title > { title } < / title > < / svg >
    }
}
