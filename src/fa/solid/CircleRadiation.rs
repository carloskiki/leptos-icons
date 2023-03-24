#[cfg(feature = "FaSolidCircleRadiation")]
use leptos::*;
#[cfg(feature = "FaSolidCircleRadiation")]
///This icon requires the feature `FaSolidCircleRadiation` to be enabled.
#[component]
pub fn CircleRadiation(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M256 64a192 192 0 1 1 0 384 192 192 0 1 1 0-384zm0 448A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM200 256c0-20.7 11.3-38.8 28-48.5l-36-62.3c-8.8-15.3-28.7-20.8-42-9c-25.6 22.6-43.9 53.3-50.9 88.1C95.7 241.5 110.3 256 128 256l72 0zm28 48.5l-36 62.4c-8.8 15.3-3.6 35.2 13.1 40.8c16 5.4 33.1 8.3 50.9 8.3s34.9-2.9 50.9-8.3c16.7-5.6 21.9-25.5 13.1-40.8l-36-62.4c-8.2 4.8-17.8 7.5-28 7.5s-19.8-2.7-28-7.5zM312 256l72 0c17.7 0 32.3-14.5 28.8-31.8c-7-34.8-25.3-65.5-50.9-88.1c-13.2-11.7-33.1-6.3-42 9l-36 62.3c16.7 9.7 28 27.8 28 48.5zm-56 32a32 32 0 1 0 0-64 32 32 0 1 0 0 64z"
        /> < title > { title } < / title > < / svg >
    }
}
