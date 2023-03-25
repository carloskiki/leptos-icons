#[cfg(feature = "BiSolidBall")]
use leptos::*;
#[cfg(feature = "BiSolidBall")]
///This icon requires the feature `BiSolidBall` to be enabled.
#[component]
pub fn Ball(
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
        "M20.247 3.034c-.069-.018-1.742-.433-4.052-.433-2.842 0-6.868.64-9.91 3.687-5.34 5.349-3.34 13.61-3.252 13.96a1 1 0 0 0 .728.726c.069.018 1.726.426 4.018.426 2.849 0 6.884-.641 9.932-3.688 5.335-5.335 3.351-13.6 3.264-13.949a1.005 1.005 0 0 0-.728-.729zm-3.537 9.262-1.414 1.414-1.793-1.792-1.586 1.586 1.792 1.793-1.414 1.414-1.792-1.793-1.793 1.793-1.414-1.414 1.793-1.793-1.793-1.794 1.414-1.414 1.793 1.794 1.586-1.586-1.794-1.793 1.414-1.414 1.794 1.793 1.793-1.793 1.414 1.414-1.793 1.793 1.793 1.792z"
        /> < title > { title } < / title > < / svg >
    }
}
