#[cfg(feature = "CgDesktop")]
use leptos::*;
#[cfg(feature = "CgDesktop")]
///This icon requires the feature `CgDesktop` to be enabled.
#[component]
pub fn Desktop(
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
        "M8 15C7.44772 15 7 15.4477 7 16C7 16.5523 7.44772 17 8 17C8.55228 17 9 16.5523 9 16C9 15.4477 8.55228 15 8 15Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11 16C11 15.4477 11.4477 15 12 15C12.5523 15 13 15.4477 13 16C13 16.5523 12.5523 17 12 17C11.4477 17 11 16.5523 11 16Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16 15C15.4477 15 15 15.4477 15 16C15 16.5523 15.4477 17 16 17C16.5523 17 17 16.5523 17 16C17 15.4477 16.5523 15 16 15Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M4 3C2.34315 3 1 4.34315 1 6V18C1 19.6569 2.34315 21 4 21H20C21.6569 21 23 19.6569 23 18V6C23 4.34315 21.6569 3 20 3H4ZM20 5H4C3.44772 5 3 5.44772 3 6V7H21V6C21 5.44772 20.5523 5 20 5ZM3 18V9H21V18C21 18.5523 20.5523 19 20 19H4C3.44772 19 3 18.5523 3 18Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
