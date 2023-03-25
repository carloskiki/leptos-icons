#[cfg(feature = "CgCalendarDue")]
use leptos::*;
#[cfg(feature = "CgCalendarDue")]
///This icon requires the feature `CgCalendarDue` to be enabled.
#[component]
pub fn CalendarDue(
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
        "M7 8C7 7.44772 7.44772 7 8 7H16C16.5523 7 17 7.44772 17 8C17 8.55228 16.5523 9 16 9H8C7.44772 9 7 8.55228 7 8Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 16C13.1046 16 14 15.1046 14 14C14 12.8954 13.1046 12 12 12C10.8954 12 10 12.8954 10 14C10 15.1046 10.8954 16 12 16Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M6 3C4.34315 3 3 4.34315 3 6V18C3 19.6569 4.34315 21 6 21H18C19.6569 21 21 19.6569 21 18V6C21 4.34315 19.6569 3 18 3H6ZM18 5H6C5.44772 5 5 5.44772 5 6V18C5 18.5523 5.44772 19 6 19H18C18.5523 19 19 18.5523 19 18V6C19 5.44772 18.5523 5 18 5Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
