#[cfg(feature = "CgBell")]
use leptos::*;
#[cfg(feature = "CgBell")]
///This icon requires the feature `CgBell` to be enabled.
#[component]
pub fn Bell(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M14 3V3.28988C16.8915 4.15043 19 6.82898 19 10V17H20V19H4V17H5V10C5 6.82898 7.10851 4.15043 10 3.28988V3C10 1.89543 10.8954 1 12 1C13.1046 1 14 1.89543 14 3ZM7 17H17V10C17 7.23858 14.7614 5 12 5C9.23858 5 7 7.23858 7 10V17ZM14 21V20H10V21C10 22.1046 10.8954 23 12 23C13.1046 23 14 22.1046 14 21Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
