#[cfg(feature = "SiWise")]
use leptos::*;
#[cfg(feature = "SiWise")]
///This icon requires the feature `SiWise` to be enabled.
#[component]
pub fn Wise(
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
        "m3.6426 0 3.7383 6.3594-6.6602 6.3613H12.043l1.1816-2.7734H7.4883l3.5879-3.588-2.084-3.5878h9.7324L9.7441 24h3.373L23.2794 0Z"
        /> < title > { title } < / title > < / svg >
    }
}
