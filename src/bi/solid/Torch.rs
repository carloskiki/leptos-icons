#[cfg(feature = "BiSolidTorch")]
use leptos::*;
#[cfg(feature = "BiSolidTorch")]
///This icon requires the feature `BiSolidTorch` to be enabled.
#[component]
pub fn Torch(
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
        "M8 11.648V20a2 2 0 0 0 2 2h4a2 2 0 0 0 2-2v-8.352c1.067-.552 3-1.928 3-4.648V5H5v2c0 2.72 1.933 4.096 3 4.648zM11 11h2v3h-2v-3zM5 2h14v2H5z"
        /> < title > { title } < / title > < / svg >
    }
}
