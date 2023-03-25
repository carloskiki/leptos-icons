#[cfg(feature = "CgToolbarTop")]
use leptos::*;
#[cfg(feature = "CgToolbarTop")]
///This icon requires the feature `CgToolbarTop` to be enabled.
#[component]
pub fn ToolbarTop(
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
        "M18 11H6V9H18V11Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M2 16C2 17.6569 3.34315 19 5 19H19C20.6569 19 22 17.6569 22 16V8C22 6.34315 20.6569 5 19 5H5C3.34315 5 2 6.34315 2 8V16ZM5 17H19C19.5523 17 20 16.5523 20 16V8C20 7.44772 19.5523 7 19 7H5C4.44772 7 4 7.44771 4 8V16C4 16.5523 4.44772 17 5 17Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
