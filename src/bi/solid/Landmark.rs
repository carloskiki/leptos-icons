#[cfg(feature = "BiSolidLandmark")]
use leptos::*;
#[cfg(feature = "BiSolidLandmark")]
///This icon requires the feature `BiSolidLandmark` to be enabled.
#[component]
pub fn Landmark(
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
        "M21 17h-2V9h2V7a.998.998 0 0 0-.594-.914l-9.432-4.191-8.421 4.21A1 1 0 0 0 2 7v2h2v8H2v4h19v-4zm-5-8v8h-3V9h3zM7 9h3v8H7V9z"
        /> < title > { title } < / title > < / svg >
    }
}
