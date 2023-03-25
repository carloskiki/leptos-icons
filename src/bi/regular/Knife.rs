#[cfg(feature = "BiRegularKnife")]
use leptos::*;
#[cfg(feature = "BiRegularKnife")]
///This icon requires the feature `BiRegularKnife` to be enabled.
#[component]
pub fn Knife(
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
        "M19.66 3.6a3 3 0 0 0-4.24 0l-.71.71-7.07 7.07 2.12 2.12-6.36 6.36 1.41 1.42L19.66 6.43c1.1-1.1 1.1-1.73.71-2.12z"
        /> < title > { title } < / title > < / svg >
    }
}
