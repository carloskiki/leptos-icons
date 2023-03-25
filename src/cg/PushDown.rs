#[cfg(feature = "CgPushDown")]
use leptos::*;
#[cfg(feature = "CgPushDown")]
///This icon requires the feature `CgPushDown` to be enabled.
#[component]
pub fn PushDown(
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
        "M11.0001 1H13.0001V15.4853L16.2428 12.2427L17.657 13.6569L12.0001 19.3137L6.34326 13.6569L7.75748 12.2427L11.0001 15.4853V1Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 20.2877H6V22.2877H18V20.2877Z" fill = "currentColor" /> < title > { title }
        < / title > < / svg >
    }
}
