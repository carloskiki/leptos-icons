#[cfg(feature = "FaSolidPersonPregnant")]
use leptos::*;
#[cfg(feature = "FaSolidPersonPregnant")]
///This icon requires the feature `FaSolidPersonPregnant` to be enabled.
#[component]
pub fn PersonPregnant(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 320 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M160 0a48 48 0 1 1 0 96 48 48 0 1 1 0-96zM88 383c-13.8-3.6-24-16.1-24-31V296.9l-4.6 7.6c-9.1 15.1-28.8 20-43.9 10.9s-20-28.8-10.9-43.9l58.3-97c15-24.9 40.3-41.5 68.7-45.6c4.1-.6 8.2-1 12.5-1h1.1 12.5H160c1.4 0 2.8 .1 4.1 .3c35.7 2.9 65.4 29.3 72.1 65l6.1 32.5c44.3 8.6 77.7 47.5 77.7 94.3v32c0 17.7-14.3 32-32 32H272 232v96c0 17.7-14.3 32-32 32s-32-14.3-32-32V384h-8-8v96c0 17.7-14.3 32-32 32s-32-14.3-32-32V383z"
        /> < title > { title } < / title > < / svg >
    }
}
