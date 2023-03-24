#[cfg(feature = "FaSolidGolfBallTee")]
use leptos::*;
#[cfg(feature = "FaSolidGolfBallTee")]
///This icon requires the feature `FaSolidGolfBallTee` to be enabled.
#[component]
pub fn GolfBallTee(
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
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M384 192c0 66.8-34.1 125.6-85.8 160H85.8C34.1 317.6 0 258.8 0 192C0 86 86 0 192 0S384 86 384 192zM242.1 256.6c0 18.5-15 33.5-33.5 33.5c-4.9 0-9.6-1.1-13.8-3c5.3 11.6 16.9 19.7 30.5 19.7c18.5 0 33.5-15 33.5-33.5c0-13.6-8.1-25.3-19.7-30.5c1.9 4.2 3 8.9 3 13.8zm-52.3-49.3c-4.9 0-9.6-1.1-13.8-3c5.3 11.6 16.9 19.7 30.5 19.7c18.5 0 33.5-15 33.5-33.5c0-13.6-8.1-25.3-19.7-30.5c1.9 4.2 3 8.9 3 13.8c0 18.5-15 33.5-33.5 33.5zm113.5-17.5c0 18.5-15 33.5-33.5 33.5c-4.9 0-9.6-1.1-13.8-3c5.3 11.6 16.9 19.7 30.5 19.7c18.5 0 33.5-15 33.5-33.5c0-13.6-8.1-25.3-19.7-30.5c1.9 4.2 3 8.9 3 13.8zM96 416c0-17.7 14.3-32 32-32h64 64c17.7 0 32 14.3 32 32s-14.3 32-32 32H240c-8.8 0-16 7.2-16 16v16c0 17.7-14.3 32-32 32s-32-14.3-32-32V464c0-8.8-7.2-16-16-16H128c-17.7 0-32-14.3-32-32z"
        /> < title > { title } < / title > < / svg >
    }
}
