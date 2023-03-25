#[cfg(feature = "BiRegularObjectsHorizontalCenter")]
use leptos::*;
#[cfg(feature = "BiRegularObjectsHorizontalCenter")]
///This icon requires the feature `BiRegularObjectsHorizontalCenter` to be enabled.
#[component]
pub fn ObjectsHorizontalCenter(
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
        "M5 20h6v2h2v-2h6a1 1 0 0 0 1-1v-5a1 1 0 0 0-1-1h-6v-2h4a1 1 0 0 0 1-1V5a1 1 0 0 0-1-1h-4V2h-2v2H7a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1h4v2H5a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1zM8 6h8v3H8zm-2 9h12v3H6z"
        /> < title > { title } < / title > < / svg >
    }
}
