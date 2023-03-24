#[cfg(feature = "BiSolidCloudLightning")]
use leptos::*;
#[cfg(feature = "BiSolidCloudLightning")]
///This icon requires the feature `BiSolidCloudLightning` to be enabled.
#[component]
pub fn CloudLightning(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M18.944 10.112C18.507 6.67 15.56 4 12 4 9.244 4 6.85 5.611 5.757 8.15 3.609 8.792 2 10.82 2 13c0 2.757 2.243 5 5 5h1.333L10 13h4l-2 3h2.975l-1.325 2H18c2.206 0 4-1.794 4-4a4.01 4.01 0 0 0-3.056-3.888zM11 18H8.333L8 19h3v3l2.649-4H11.5z"
        /> < title > { title } < / title > < / svg >
    }
}
