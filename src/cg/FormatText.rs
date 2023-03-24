#[cfg(feature = "CgFormatText")]
use leptos::*;
#[cfg(feature = "CgFormatText")]
///This icon requires the feature `CgFormatText` to be enabled.
#[component]
pub fn FormatText(
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
        "M6.5 3H3V6.5H4V4H6.5V3Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8.5 4V3H11V4H8.5Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M13 4H15.5V3H13V4Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.5 3V4H20V6.5H21V3H17.5Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 8.5H20V11H21V8.5Z" fill = "currentColor"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M21 13H20V15.5H21V13Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 17.5H20V20H17.5V21H21V17.5Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15.5 21V20H13V21H15.5Z" fill = "currentColor"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M11 21V20H8.5V21H11Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.5 21V20H4V17.5H3V21H6.5Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 15.5H4V13H3V15.5Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M3 11H4V8.5H3V11Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11 9.5H7V7.5H17V9.5H13V16.5H11V9.5Z" fill = "currentColor" /> < title > { title
        } < / title > < / svg >
    }
}
