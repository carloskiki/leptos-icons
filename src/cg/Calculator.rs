#[cfg(feature = "CgCalculator")]
use leptos::*;
#[cfg(feature = "CgCalculator")]
///This icon requires the feature `CgCalculator` to be enabled.
#[component]
pub fn Calculator(
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
        "M17 5H7V7H17V5Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 9H9V11H7V9Z" fill = "currentColor" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M9 13H7V15H9V13Z" fill = "currentColor"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M7 17H9V19H7V17Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13 9H11V11H13V9Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 13H13V15H11V13Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M13 17H11V19H13V17Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 9H17V11H15V9Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 13H15V19H17V13Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule =
        "evenodd" d =
        "M3 3C3 1.89543 3.89543 1 5 1H19C20.1046 1 21 1.89543 21 3V21C21 22.1046 20.1046 23 19 23H5C3.89543 23 3 22.1046 3 21V3ZM5 3H19V21H5V3Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
