#[cfg(feature = "OcLgKey")]
use leptos::*;
#[cfg(feature = "OcLgKey")]
///This icon requires the feature `OcLgKey` to be enabled.
#[component]
pub fn Key(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M16.75 8.5a1.25 1.25 0 1 0 0-2.5 1.25 1.25 0 0 0 0 2.5Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M15.75 0a8.25 8.25 0 1 1-2.541 16.101l-1.636 1.636a1.744 1.744 0 0 1-1.237.513H9.25a.25.25 0 0 0-.25.25v1.448a.876.876 0 0 1-.256.619l-.214.213a.75.75 0 0 1-.545.22H5.25a.25.25 0 0 0-.25.25v1A1.75 1.75 0 0 1 3.25 24h-1.5A1.75 1.75 0 0 1 0 22.25v-2.836c0-.464.185-.908.513-1.236l7.386-7.388A8.249 8.249 0 0 1 15.75 0ZM9 8.25a6.733 6.733 0 0 0 .463 2.462.75.75 0 0 1-.168.804l-7.722 7.721a.25.25 0 0 0-.073.177v2.836c0 .138.112.25.25.25h1.5a.25.25 0 0 0 .25-.25v-1c0-.966.784-1.75 1.75-1.75H7.5v-1c0-.966.784-1.75 1.75-1.75h1.086a.25.25 0 0 0 .177-.073l1.971-1.972a.75.75 0 0 1 .804-.168A6.75 6.75 0 1 0 9 8.25Z"
        /> < title > { title } < / title > < / svg >
    }
}
