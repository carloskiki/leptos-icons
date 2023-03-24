#[cfg(feature = "CgClipboard")]
use leptos::*;
#[cfg(feature = "CgClipboard")]
///This icon requires the feature `CgClipboard` to be enabled.
#[component]
pub fn Clipboard(
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
        "M8 11C7.44772 11 7 11.4477 7 12C7 12.5523 7.44772 13 8 13H15.9595C16.5118 13 16.9595 12.5523 16.9595 12C16.9595 11.4477 16.5118 11 15.9595 11H8Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.04053 15.0665C7.48824 15.0665 7.04053 15.5142 7.04053 16.0665C7.04053 16.6188 7.48824 17.0665 8.04053 17.0665H16C16.5523 17.0665 17 16.6188 17 16.0665C17 15.5142 16.5523 15.0665 16 15.0665H8.04053Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M5 3C3.89543 3 3 3.89543 3 5V19C3 20.1046 3.89543 21 5 21H19C20.1046 21 21 20.1046 21 19V5C21 3.89543 20.1046 3 19 3H5ZM7 5H5L5 19H19V5H17V6C17 7.65685 15.6569 9 14 9H10C8.34315 9 7 7.65685 7 6V5ZM9 5V6C9 6.55228 9.44772 7 10 7H14C14.5523 7 15 6.55228 15 6V5H9Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
