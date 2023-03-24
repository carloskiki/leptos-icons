#[cfg(feature = "TiWine")]
use leptos::*;
#[cfg(feature = "TiWine")]
///This icon requires the feature `TiWine` to be enabled.
#[component]
pub fn Wine(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = { size.clone() } height = { size
        } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.568 9.432c0-2.55-.906-5.592-.944-5.72-.128-.423-.517-.712-.958-.712h-7.332c-.441 0-.83.289-.958.712-.038.128-.944 3.17-.944 5.72 0 2.735 1.984 5.011 4.587 5.477l-.019.091v4h-1c-.553 0-1 .447-1 1s.447 1 1 1h4c.553 0 1-.447 1-1s-.447-1-1-1h-1v-4l-.019-.092c2.603-.466 4.587-2.741 4.587-5.476zm-5.568 3.568c-1.773 0-3.236-1.303-3.511-3h7.021c-.274 1.697-1.737 3-3.51 3zm-3.555-4c.062-1.468.422-3.093.653-4h5.803c.231.907.591 2.532.653 4h-7.109z"
        /> < title > { title } < / title > < / svg >
    }
}
