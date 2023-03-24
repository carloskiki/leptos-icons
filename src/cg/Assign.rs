#[cfg(feature = "CgAssign")]
use leptos::*;
#[cfg(feature = "CgAssign")]
///This icon requires the feature `CgAssign` to be enabled.
#[component]
pub fn Assign(
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
        "M6 6H10V4H4V10H6V6Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 18H6V14H4V20H10V18Z" fill = "currentColor"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M14 6H18V10H20V4H14V6Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14 18H18V14H20V20H14V18Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 8.5C10.067 8.5 8.5 10.067 8.5 12C8.5 13.933 10.067 15.5 12 15.5C13.933 15.5 15.5 13.933 15.5 12C15.5 10.067 13.933 8.5 12 8.5Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
