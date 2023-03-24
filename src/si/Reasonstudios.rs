#[cfg(feature = "SiReasonstudios")]
use leptos::*;
#[cfg(feature = "SiReasonstudios")]
///This icon requires the feature `SiReasonstudios` to be enabled.
#[component]
pub fn Reasonstudios(
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
        "M2.49 5.114l8.3-4.79a2.421 2.421 0 012.39-.017l.03.017 8.299 4.79c.74.427 1.2 1.212 1.211 2.065V16.79c0 .854-.451 1.645-1.184 2.08l-.027.016-8.299 4.79a2.42 2.42 0 01-2.39.017l-.03-.017-8.3-4.79a2.421 2.421 0 01-1.21-2.065V7.21c0-.855.45-1.645 1.184-2.08l.026-.016 8.3-4.79zM12 4.026L5.092 8.013v7.974L12 19.974V12l6.908-3.987z"
        /> < title > { title } < / title > < / svg >
    }
}
