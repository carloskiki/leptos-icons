#[cfg(feature = "OcSmArrowDownLeft")]
use leptos::*;
#[cfg(feature = "OcSmArrowDownLeft")]
///This icon requires the feature `OcSmArrowDownLeft` to be enabled.
#[component]
pub fn ArrowDownLeft(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M11.78 4.22a.75.75 0 0 1 0 1.06l-5.26 5.26h4.2a.75.75 0 0 1 0 1.5H4.71a.75.75 0 0 1-.75-.75V5.28a.75.75 0 0 1 1.5 0v4.2l5.26-5.26a.75.75 0 0 1 1.06 0Z"
        /> < title > { title } < / title > < / svg >
    }
}
