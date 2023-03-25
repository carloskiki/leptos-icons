#[cfg(feature = "BiSolidObjectsVerticalCenter")]
use leptos::*;
#[cfg(feature = "BiSolidObjectsVerticalCenter")]
///This icon requires the feature `BiSolidObjectsVerticalCenter` to be enabled.
#[component]
pub fn ObjectsVerticalCenter(
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
        "M19 7a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v4h-2V5a1 1 0 0 0-1-1H6a1 1 0 0 0-1 1v6H2v2h3v6a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1v-6h2v4a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1v-4h3v-2h-3z"
        /> < title > { title } < / title > < / svg >
    }
}
