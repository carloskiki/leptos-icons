#[cfg(feature = "IoArrowRedo")]
use leptos::*;
#[cfg(feature = "IoArrowRedo")]
///This icon requires the feature `IoArrowRedo` to be enabled.
#[component]
pub fn ArrowRedo(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M58.79,439.13A16,16,0,0,1,48,424c0-73.1,14.68-131.56,43.65-173.77,35-51,90.21-78.46,164.35-81.87V88a16,16,0,0,1,27.05-11.57l176,168a16,16,0,0,1,0,23.14l-176,168A16,16,0,0,1,256,424V344.23c-45,1.36-79,8.65-106.07,22.64-29.25,15.12-50.46,37.71-73.32,67a16,16,0,0,1-17.82,5.28Z"
        /> < title > { title } < / title > < / svg >
    }
}
