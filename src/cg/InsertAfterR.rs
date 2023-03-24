#[cfg(feature = "CgInsertAfterR")]
use leptos::*;
#[cfg(feature = "CgInsertAfterR")]
///This icon requires the feature `CgInsertAfterR` to be enabled.
#[component]
pub fn InsertAfterR(
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
        "M9 8C8.44772 8 8 8.44772 8 9C8 9.55229 8.44772 10 9 10H11V12C11 12.5523 11.4477 13 12 13C12.5523 13 13 12.5523 13 12V10H15C15.5523 10 16 9.55229 16 9C16 8.44772 15.5523 8 15 8H13V6C13 5.44772 12.5523 5 12 5C11.4477 5 11 5.44772 11 6V8H9Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M4 4C4 2.34315 5.34315 1 7 1H17C18.6569 1 20 2.34315 20 4V14C20 15.6569 18.6569 17 17 17H7C5.34315 17 4 15.6569 4 14V4ZM7 3H17C17.5523 3 18 3.44772 18 4V14C18 14.5523 17.5523 15 17 15H7C6.44772 15 6 14.5523 6 14V4C6 3.44772 6.44772 3 7 3Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 20C4.44772 20 4 20.4477 4 21C4 21.5523 4.44772 22 5 22H19C19.5523 22 20 21.5523 20 21C20 20.4477 19.5523 20 19 20H5Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
