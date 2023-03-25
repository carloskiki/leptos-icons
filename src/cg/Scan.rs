#[cfg(feature = "CgScan")]
use leptos::*;
#[cfg(feature = "CgScan")]
///This icon requires the feature `CgScan` to be enabled.
#[component]
pub fn Scan(
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
        "M11 3H13V21H11V3Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M5 8C5 7.44771 5.44772 7 6 7H9V5H6C4.34315 5 3 6.34315 3 8V16C3 17.6569 4.34315 19 6 19H9V17H6C5.44772 17 5 16.5523 5 16V8Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 8C19 7.44771 18.5523 7 18 7H15V5H18C19.6569 5 21 6.34315 21 8V16C21 17.6569 19.6569 19 18 19H15V17H18C18.5523 17 19 16.5523 19 16V8Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
