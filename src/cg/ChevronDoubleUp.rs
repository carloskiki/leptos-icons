#[cfg(feature = "CgChevronDoubleUp")]
use leptos::*;
#[cfg(feature = "CgChevronDoubleUp")]
///This icon requires the feature `CgChevronDoubleUp` to be enabled.
#[component]
pub fn ChevronDoubleUp(
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
        "M17.6569 11.2929L16.2427 12.7071L12 8.46444L7.75735 12.7071L6.34314 11.2929L12 5.63605L17.6569 11.2929Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.6569 16.9497L16.2427 18.3639L12 14.1213L7.75735 18.364L6.34314 16.9498L12 11.2929L17.6569 16.9497Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
