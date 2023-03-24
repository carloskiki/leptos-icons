#[cfg(feature = "SiNano")]
use leptos::*;
#[cfg(feature = "SiNano")]
///This icon requires the feature `SiNano` to be enabled.
#[component]
pub fn Nano(
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
        "M22.2864 6.8576c-.9453 0-1.7135.7665-1.7135 1.7136 0 1.2843-.4275 1.7136-1.7136 1.7136-.9453 0-1.7135.7664-1.7135 1.7135 0 1.2843-.4276 1.7136-1.7136 1.7136-.9453 0-1.7135.7664-1.7135 1.7135 0 .9454.7665 1.7136 1.7135 1.7136.9454 0 1.7136-.7665 1.7136-1.7136 0-1.2843.4275-1.7135 1.7135-1.7135.9454 0 1.7136-.7665 1.7136-1.7136 0-1.2843.4275-1.7135 1.7135-1.7135.9454 0 1.7136-.7666 1.7136-1.7136 0-.9454-.7682-1.7136-1.7136-1.7136zm-13.717.0017c-.9453 0-1.7135.7665-1.7135 1.7136 0 1.2843-.4275 1.7136-1.7135 1.7136-.9454 0-1.7136.7664-1.7136 1.7135 0 .947.77 1.7135 1.7153 1.7135S6.8576 12.9471 6.8576 12c0-1.2843.4293-1.7135 1.7136-1.7135s1.7136.4275 1.7136 1.7135c0 .947.7698 1.7135 1.7152 1.7135.9453 0 1.7135-.7664 1.7135-1.7135 0-.9454-.7664-1.7135-1.7169-1.7135-1.2843 0-1.7135-.4276-1.7135-1.7136 0-.9453-.7683-1.7136-1.7136-1.7136zm-6.8559 6.856A1.7136 1.7136 0 0 0 0 15.4287a1.7136 1.7136 0 0 0 1.7135 1.7136 1.7136 1.7136 0 0 0 1.7136-1.7136 1.7136 1.7136 0 0 0-1.7135-1.7136Z"
        /> < title > { title } < / title > < / svg >
    }
}
