#[cfg(feature = "CgGlass")]
use leptos::*;
#[cfg(feature = "CgGlass")]
///This icon requires the feature `CgGlass` to be enabled.
#[component]
pub fn Glass(
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
        "M17 10C17 12.419 15.2822 14.4367 13 14.9V17H15V19H9V17H11V14.9C8.71776 14.4367 7 12.419 7 10V5H17V10ZM15 7H9V10C9 11.6569 10.3431 13 12 13C13.6569 13 15 11.6569 15 10V7Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
