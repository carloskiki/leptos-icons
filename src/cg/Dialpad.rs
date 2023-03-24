#[cfg(feature = "CgDialpad")]
use leptos::*;
#[cfg(feature = "CgDialpad")]
///This icon requires the feature `CgDialpad` to be enabled.
#[component]
pub fn Dialpad(
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
        "M5.5 3H8.5V6H5.5V3Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5.5 8H8.5V11H5.5V8Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M5.5 13V16H8.5V13H5.5Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.5 3H13.5V6H10.5V3Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10.5 8V11H13.5V8H10.5Z" fill = "currentColor"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M10.5 13H13.5V16H10.5V13Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.5 18V21H13.5V18H10.5Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15.5 3H18.5V6H15.5V3Z" fill = "currentColor"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M15.5 8V11H18.5V8H15.5Z" fill
        = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.5 13H18.5V16H15.5V13Z" fill = "currentColor" /> < title > { title } < /
        title > < / svg >
    }
}
