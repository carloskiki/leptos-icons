#[cfg(feature = "SiMoscowmetro")]
use leptos::*;
#[cfg(feature = "SiMoscowmetro")]
///This icon requires the feature `SiMoscowmetro` to be enabled.
#[component]
pub fn Moscowmetro(
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
        "M16.603 11.85l-2.481-6.26-2.092 3.66-2.092-3.66-2.481 6.262H6.74v.941h3.736v-.941h-.553l.538-1.555 1.569 2.57 1.569-2.57.538 1.555h-.553v.941h3.751v-.941zm5.335-1.912A9.933 9.933 0 0 0 12 0C6.516 0 2.062 4.453 2.062 9.938c0 2.75 1.121 5.23 2.914 7.023a.804.804 0 0 0 1.375-.568.825.825 0 0 0-.239-.582 8.303 8.303 0 0 1-2.42-5.873c0-4.588 3.72-8.324 8.308-8.324 4.588 0 8.324 3.736 8.324 8.324a8.289 8.289 0 0 1-2.436 5.888l-7.024 7.023L12 24l7.039-7.039a9.891 9.891 0 0 0 2.899-7.023Z"
        /> < title > { title } < / title > < / svg >
    }
}
