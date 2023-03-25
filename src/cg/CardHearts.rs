#[cfg(feature = "CgCardHearts")]
use leptos::*;
#[cfg(feature = "CgCardHearts")]
///This icon requires the feature `CgCardHearts` to be enabled.
#[component]
pub fn CardHearts(
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
        "M9.14648 12.2929C8.36544 11.5118 8.36544 10.2455 9.14648 9.46444C9.92753 8.68339 11.1939 8.68339 11.9749 9.46444L12 9.48955L12.0251 9.46449C12.8061 8.68345 14.0725 8.68345 14.8535 9.46449C15.6346 10.2455 15.6346 11.5119 14.8535 12.2929L12.0251 15.1213L12 15.0962L11.9749 15.1213L9.14648 12.2929Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M3 4C3 2.34315 4.34315 1 6 1H18C19.6569 1 21 2.34315 21 4V20C21 21.6569 19.6569 23 18 23H6C4.34315 23 3 21.6569 3 20V4ZM6 3H18C18.5523 3 19 3.44772 19 4V20C19 20.5523 18.5523 21 18 21H6C5.44772 21 5 20.5523 5 20V4C5 3.44772 5.44772 3 6 3Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
