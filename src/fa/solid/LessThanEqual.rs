#[cfg(feature = "FaSolidLessThanEqual")]
use leptos::*;
#[cfg(feature = "FaSolidLessThanEqual")]
///This icon requires the feature `FaSolidLessThanEqual` to be enabled.
#[component]
pub fn LessThanEqual(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M395.9 93.7c16.4-6.6 24.4-25.2 17.8-41.6s-25.2-24.4-41.6-17.8l-320 128C40 167.1 32 178.9 32 192s8 24.9 20.1 29.7l320 128c16.4 6.6 35-1.4 41.6-17.8s-1.4-35-17.8-41.6L150.2 192 395.9 93.7zM32 416c-17.7 0-32 14.3-32 32s14.3 32 32 32H416c17.7 0 32-14.3 32-32s-14.3-32-32-32H32z"
        /> < title > { title } < / title > < / svg >
    }
}
