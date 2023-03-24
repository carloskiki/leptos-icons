#[cfg(feature = "CgInsertAfterO")]
use leptos::*;
#[cfg(feature = "CgInsertAfterO")]
///This icon requires the feature `CgInsertAfterO` to be enabled.
#[component]
pub fn InsertAfterO(
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
        "M4 9C4 4.58172 7.58172 1 12 1C16.4183 1 20 4.58172 20 9C20 13.4183 16.4183 17 12 17C7.58172 17 4 13.4183 4 9ZM12 15C8.68629 15 6 12.3137 6 9C6 5.68629 8.68629 3 12 3C15.3137 3 18 5.68629 18 9C18 12.3137 15.3137 15 12 15Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 20C4.44772 20 4 20.4477 4 21C4 21.5523 4.44772 22 5 22H19C19.5523 22 20 21.5523 20 21C20 20.4477 19.5523 20 19 20H5Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
