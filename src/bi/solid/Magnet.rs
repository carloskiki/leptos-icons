#[cfg(feature = "BiSolidMagnet")]
use leptos::*;
#[cfg(feature = "BiSolidMagnet")]
///This icon requires the feature `BiSolidMagnet` to be enabled.
#[component]
pub fn Magnet(
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
        "M8 3H5a1 1 0 0 0-1 1v3h5V4a1 1 0 0 0-1-1zm7 1v3h5V4a1 1 0 0 0-1-1h-3a1 1 0 0 0-1 1zm0 10a3 3 0 0 1-6 0V9H4v5a8 8 0 0 0 16 0V9h-5v5z"
        /> < title > { title } < / title > < / svg >
    }
}
