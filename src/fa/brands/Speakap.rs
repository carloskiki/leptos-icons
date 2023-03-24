#[cfg(feature = "FaBrandsSpeakap")]
use leptos::*;
#[cfg(feature = "FaBrandsSpeakap")]
///This icon requires the feature `FaBrandsSpeakap` to be enabled.
#[component]
pub fn Speakap(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M64 391.78C-15.41 303.59-8 167.42 80.64 87.64s224.8-73 304.21 15.24 72 224.36-16.64 304.14c-18.74 16.87 64 43.09 42 52.26-82.06 34.21-253.91 35-346.23-67.5zm213.31-211.6l38.5-40.86c-9.61-8.89-32-26.83-76.17-27.6-52.33-.91-95.86 28.3-96.77 80-.2 11.33.29 36.72 29.42 54.83 34.46 21.42 86.52 21.51 86 52.26-.37 21.28-26.42 25.81-38.59 25.6-3-.05-30.23-.46-47.61-24.62l-40 42.61c28.16 27 59 32.62 83.49 33.05 10.23.18 96.42.33 97.84-81 .28-15.81-2.07-39.72-28.86-56.59-34.36-21.64-85-19.45-84.43-49.75.41-23.25 31-25.37 37.53-25.26.43 0 26.62.26 39.62 17.37z"
        /> < title > { title } < / title > < / svg >
    }
}
