#[cfg(feature = "SiSparkpost")]
use leptos::*;
#[cfg(feature = "SiSparkpost")]
///This icon requires the feature `SiSparkpost` to be enabled.
#[component]
pub fn Sparkpost(
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
        "M16.2 9c-1.351.9-1.8 2.7-1.65 3.9-2.25-2.25 3.45-8.55-3-12.9C15.15 5.4 6 9.75 6 17.4c0 3 1.95 5.701 6 6.6 4.05-.898 6-3.6 6-6.6 0-4.5-2.7-6-1.8-8.4zM12 20.852c-1.8 0-3.45-1.5-3.45-3.451 0-1.801 1.5-3.45 3.45-3.45 1.8 0 3.45 1.5 3.45 3.45-.15 1.951-1.65 3.451-3.45 3.451z"
        /> < title > { title } < / title > < / svg >
    }
}
