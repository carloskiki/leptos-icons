#[cfg(feature = "CgInsertBefore")]
use leptos::*;
#[cfg(feature = "CgInsertBefore")]
///This icon requires the feature `CgInsertBefore` to be enabled.
#[component]
pub fn InsertBefore(
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
        "M3 5C3 5.55228 3.44772 6 4 6H20C20.5523 6 21 5.55228 21 5C21 4.44772 20.5523 4 20 4H4C3.44772 4 3 4.44772 3 5Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 20C12.5523 20 13 19.5523 13 19V16H16C16.5523 16 17 15.5523 17 15C17 14.4477 16.5523 14 16 14H13V11C13 10.4477 12.5523 10 12 10C11.4477 10 11 10.4477 11 11V14H8C7.44772 14 7 14.4477 7 15C7 15.5523 7.44772 16 8 16H11V19C11 19.5523 11.4477 20 12 20Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
