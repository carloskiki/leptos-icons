#[cfg(feature = "SiCloudsmith")]
use leptos::*;
#[cfg(feature = "SiCloudsmith")]
///This icon requires the feature `SiCloudsmith` to be enabled.
#[component]
pub fn Cloudsmith(
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
        "M16.15 0a4.146 4.146 0 0 0-2.94 1.225c-.981.98-1.34 2.288-1.177 3.53-.458 2.548-2.843 2.908-3.889 2.94-1.176-.098-2.352.327-3.235 1.21a4.142 4.142 0 0 0 0 5.88 4.142 4.142 0 0 0 5.882 0A4.136 4.136 0 0 0 12 12.108v-.23c.097-3.104 2.777-3.529 3.92-3.561h.523c.98-.066 1.928-.458 2.647-1.21a4.142 4.142 0 0 0 0-5.88A4.146 4.146 0 0 0 16.15 0zm-.327 15.7a4.15 4.15 0 0 0-4.15 4.15 4.15 4.15 0 0 0 4.15 4.15 4.15 4.15 0 0 0 4.15-4.15 4.15 4.15 0 0 0-4.15-4.15z"
        /> < title > { title } < / title > < / svg >
    }
}
