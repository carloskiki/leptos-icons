#[cfg(feature = "CgVolume")]
use leptos::*;
#[cfg(feature = "CgVolume")]
///This icon requires the feature `CgVolume` to be enabled.
#[component]
pub fn Volume(
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
        "M24 12C24 16.4183 20.4183 20 16 20V18C19.3137 18 22 15.3137 22 12C22 8.68629 19.3137 6 16 6V4C20.4183 4 24 7.58172 24 12Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 12C20 14.2091 18.2091 16 16 16V14C17.1046 14 18 13.1046 18 12C18 10.8954 17.1046 10 16 10V8C18.2091 8 20 9.79086 20 12Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M9 16L15 20V4L9 8H5C2.79086 8 1 9.79086 1 12C1 14.2091 2.79086 16 5 16H9ZM5 10H9L13 7.5V16.5L9 14H5C3.89543 14 3 13.1046 3 12C3 10.8954 3.89543 10 5 10Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
