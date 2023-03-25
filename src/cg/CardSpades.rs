#[cfg(feature = "CgCardSpades")]
use leptos::*;
#[cfg(feature = "CgCardSpades")]
///This icon requires the feature `CgCardSpades` to be enabled.
#[component]
pub fn CardSpades(
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
        "M9.14648 11.7071C8.36544 12.4882 8.36544 13.7545 9.14648 14.5356C9.92753 15.3166 11.1939 15.3166 11.9749 14.5356L12 14.5104L12.0251 14.5355C12.8061 15.3166 14.0725 15.3166 14.8535 14.5355C15.6346 13.7545 15.6346 12.4881 14.8535 11.7071L12.0251 8.87865L12 8.90377L11.9749 8.87871L9.14648 11.7071Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M3 20C3 21.6569 4.34315 23 6 23H18C19.6569 23 21 21.6569 21 20V4C21 2.34315 19.6569 1 18 1H6C4.34315 1 3 2.34315 3 4V20ZM6 21H18C18.5523 21 19 20.5523 19 20V4C19 3.44772 18.5523 3 18 3H6C5.44772 3 5 3.44772 5 4V20C5 20.5523 5.44772 21 6 21Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
