#[cfg(feature = "SiMarriott")]
use leptos::*;
#[cfg(feature = "SiMarriott")]
///This icon requires the feature `SiMarriott` to be enabled.
#[component]
pub fn Marriott(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M8.802 11.083l-1.178 2.41c-.8 1.425-1.931 3.167-3.646 3.603-.668.232-1.255.023-1.9-.023L0 20.476a1.626 1.626 0 0 0 .59.386c3.647 1.39 5.122-.1 8.722-8.238l3.403 7.249h4.53l-2.14-4.893 1.213-2.53 3.345 7.311 4.337.027-7.59-16.677-3.475 1.738 2.738 6.222-1.201 2.445L9.45 2.678l-3.7 1.877Z"
        /> < title > { title } < / title > < / svg >
    }
}
