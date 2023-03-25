#[cfg(feature = "CgGhostCharacter")]
use leptos::*;
#[cfg(feature = "CgGhostCharacter")]
///This icon requires the feature `CgGhostCharacter` to be enabled.
#[component]
pub fn GhostCharacter(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.9759 9C10.9759 9.55228 10.5282 10 9.97589 10C9.42361 10 8.97589 9.55228 8.97589 9C8.97589 8.44771 9.42361 8 9.97589 8C10.5282 8 10.9759 8.44771 10.9759 9Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13.9712 10C14.5235 10 14.9712 9.55228 14.9712 9C14.9712 8.44771 14.5235 8 13.9712 8C13.4189 8 12.9712 8.44771 12.9712 9C12.9712 9.55228 13.4189 10 13.9712 10Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M19 20.9999V10C19 6.13401 15.866 3 12 3C8.13401 3 5 6.13401 5 10V21L7.82846 21L9.24264 19.5858L10.6569 21L13.3433 21L14.7574 19.5858L16.1717 21L19 20.9999ZM17 10C17 7.23858 14.7614 5 12 5C9.23858 5 7 7.23858 7 10V19L9.24264 16.7573L12 19.5147L14.7574 16.7573L17 18.9999V10Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
