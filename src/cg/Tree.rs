#[cfg(feature = "CgTree")]
use leptos::*;
#[cfg(feature = "CgTree")]
///This icon requires the feature `CgTree` to be enabled.
#[component]
pub fn Tree(
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
        "M11 17.9C8.71776 17.4367 7 15.419 7 13V7C7 4.23858 9.23858 2 12 2C14.7614 2 17 4.23858 17 7V13C17 15.419 15.2822 17.4367 13 17.9V21C13 21.5523 12.5523 22 12 22C11.4477 22 11 21.5523 11 21V17.9ZM12 4C13.6569 4 15 5.34315 15 7V13C15 14.3062 14.1652 15.4175 13 15.8293V11C13 10.4477 12.5523 10 12 10C11.4477 10 11 10.4477 11 11V15.8293C9.83481 15.4175 9 14.3062 9 13V7C9 5.34315 10.3431 4 12 4Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
