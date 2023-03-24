#[cfg(feature = "CgController")]
use leptos::*;
#[cfg(feature = "CgController")]
///This icon requires the feature `CgController` to be enabled.
#[component]
pub fn Controller(
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
        "M14.8284 6.34313L16.2426 4.92892L12 0.686279L7.75735 4.92892L9.17156 6.34313L12 3.51471L14.8284 6.34313Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4.92892 16.2426L6.34313 14.8284L3.51471 12L6.34313 9.17156L4.92892 7.75735L0.686279 12L4.92892 16.2426Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.75735 19.0711L12 23.3137L16.2426 19.0711L14.8284 17.6568L12 20.4853L9.17156 17.6568L7.75735 19.0711Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.6568 9.17156L20.4853 12L17.6568 14.8284L19.0711 16.2426L23.3137 12L19.0711 7.75735L17.6568 9.17156Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M12 8C14.2091 8 16 9.79086 16 12C16 14.2091 14.2091 16 12 16C9.79086 16 8 14.2091 8 12C8 9.79086 9.79086 8 12 8ZM12 10C13.1046 10 14 10.8954 14 12C14 13.1046 13.1046 14 12 14C10.8954 14 10 13.1046 10 12C10 10.8954 10.8954 10 12 10Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
