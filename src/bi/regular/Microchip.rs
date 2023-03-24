#[cfg(feature = "BiRegularMicrochip")]
use leptos::*;
#[cfg(feature = "BiRegularMicrochip")]
///This icon requires the feature `BiRegularMicrochip` to be enabled.
#[component]
pub fn Microchip(
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
        "M16 2H8c-1.103 0-2 .897-2 2v16c0 1.103.897 2 2 2h8c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zM8 20V4h8l.001 16H8zM3 7h2V5H3v.5H2v1h1zm18-2h-2v2h2v-.5h1v-1h-1zM3 11h2V9H3v.5H2v1h1zm18-2h-2v2h2v-.5h1v-1h-1zM3 15h2v-2H3v.5H2v1h1zm18-2h-2v2h2v-.5h1v-1h-1zM3 19h2v-2H3v.5H2v1h1zm18-2h-2v2h2v-.5h1v-1h-1z"
        /> < title > { title } < / title > < / svg >
    }
}
