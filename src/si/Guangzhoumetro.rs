#[cfg(feature = "SiGuangzhoumetro")]
use leptos::*;
#[cfg(feature = "SiGuangzhoumetro")]
///This icon requires the feature `SiGuangzhoumetro` to be enabled.
#[component]
pub fn Guangzhoumetro(
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
        "M16.433 12.329A16.188 16.188 0 0 1 22.118.009L17.684 0a16.2 16.2 0 0 0-4.776 11.374V24h3.525zm-8.869 0A16.174 16.174 0 0 0 1.882.009L6.319 0a16.238 16.238 0 0 1 4.773 11.374V24H7.564z"
        /> < title > { title } < / title > < / svg >
    }
}
