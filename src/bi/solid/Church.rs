#[cfg(feature = "BiSolidChurch")]
use leptos::*;
#[cfg(feature = "BiSolidChurch")]
///This icon requires the feature `BiSolidChurch` to be enabled.
#[component]
pub fn Church(
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
        "M10 22v-4a2 2 0 0 1 4 0v4h4V12a1 1 0 0 0-.485-.857L13 8.434V6h2V4h-2V2h-2v2H9v2h2v2.434l-4.515 2.709A1 1 0 0 0 6 12v10h4zm-7 0h2v-8.118l-2.447 1.224A.998.998 0 0 0 2 16v5a1 1 0 0 0 1 1zm18.447-6.895L19 13.882V22h2a1 1 0 0 0 1-1v-5c0-.379-.214-.725-.553-.895z"
        /> < title > { title } < / title > < / svg >
    }
}
