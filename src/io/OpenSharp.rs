#[cfg(feature = "IoOpenSharp")]
use leptos::*;
#[cfg(feature = "IoOpenSharp")]
///This icon requires the feature `IoOpenSharp` to be enabled.
#[component]
pub fn OpenSharp(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < polygon xmlns =
        "http://www.w3.org/2000/svg" points =
        "201.37 288 377.37 112 48 112 48 464 400 464 400 134.63 224 310.63 201.37 288"
        />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "320 48 320 80 409.37 80 377.37 112 400 134.63 432 102.63 432 192 464 192 464 48 320 48"
        /> < title > { title } < / title > < / svg >
    }
}
