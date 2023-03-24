#[cfg(feature = "CgInsertBeforeR")]
use leptos::*;
#[cfg(feature = "CgInsertBeforeR")]
///This icon requires the feature `CgInsertBeforeR` to be enabled.
#[component]
pub fn InsertBeforeR(
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
        "M5 3C4.44772 3 4 2.55228 4 2C4 1.44772 4.44772 1 5 1H19C19.5523 1 20 1.44772 20 2C20 2.55228 19.5523 3 19 3H5Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 15C8.44772 15 8 14.5523 8 14C8 13.4477 8.44772 13 9 13H11V11C11 10.4477 11.4477 10 12 10C12.5523 10 13 10.4477 13 11V13H15C15.5523 13 16 13.4477 16 14C16 14.5523 15.5523 15 15 15H13V17C13 17.5523 12.5523 18 12 18C11.4477 18 11 17.5523 11 17V15H9Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M4 19C4 20.6569 5.34315 22 7 22H17C18.6569 22 20 20.6569 20 19V9C20 7.34315 18.6569 6 17 6H7C5.34315 6 4 7.34315 4 9V19ZM17 20C17.5523 20 18 19.5523 18 19V9C18 8.44772 17.5523 8 17 8H7C6.44772 8 6 8.44772 6 9V19C6 19.5523 6.44772 20 7 20H17Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
