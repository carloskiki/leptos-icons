#[cfg(feature = "CgMouse")]
use leptos::*;
#[cfg(feature = "CgMouse")]
///This icon requires the feature `CgMouse` to be enabled.
#[component]
pub fn Mouse(
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
        "M12 5C11.4477 5 11 5.44772 11 6V10C11 10.5523 11.4477 11 12 11C12.5523 11 13 10.5523 13 10V6C13 5.44772 12.5523 5 12 5Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M4 8C4 3.58172 7.58172 0 12 0C16.4183 0 20 3.58172 20 8V16C20 20.4183 16.4183 24 12 24C7.58172 24 4 20.4183 4 16V8ZM18 8V16C18 19.3137 15.3137 22 12 22C8.68629 22 6 19.3137 6 16V8C6 4.68629 8.68629 2 12 2C15.3137 2 18 4.68629 18 8Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
