#[cfg(feature = "FaSolidMitten")]
use leptos::*;
#[cfg(feature = "FaSolidMitten")]
///This icon requires the feature `FaSolidMitten` to be enabled.
#[component]
pub fn Mitten(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M352 384H64L5.4 178.9C1.8 166.4 0 153.4 0 140.3C0 62.8 62.8 0 140.3 0h3.4c66 0 123.5 44.9 139.5 108.9l31.4 125.8 17.6-20.1C344.8 200.2 362.9 192 382 192h2.8c34.9 0 63.3 28.3 63.3 63.3c0 15.9-6 31.2-16.8 42.9L352 384zM32 448c0-17.7 14.3-32 32-32H352c17.7 0 32 14.3 32 32v32c0 17.7-14.3 32-32 32H64c-17.7 0-32-14.3-32-32V448z"
        /> < title > { title } < / title > < / svg >
    }
}
