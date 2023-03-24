#[cfg(feature = "FaSolidKiwiBird")]
use leptos::*;
#[cfg(feature = "FaSolidKiwiBird")]
///This icon requires the feature `FaSolidKiwiBird` to be enabled.
#[component]
pub fn KiwiBird(
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
        stroke_witdh = "0" style = style viewBox = "0 0 576 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M291.2 388.4c31.2-18.8 64.7-36.4 101.1-36.4H448c4.6 0 9.1-.2 13.6-.7l85.3 121.9c4 5.7 11.3 8.2 17.9 6.1s11.2-8.3 11.2-15.3V224c0-70.7-57.3-128-128-128H392.3c-36.4 0-69.9-17.6-101.1-36.4C262.3 42.1 228.3 32 192 32C86 32 0 118 0 224c0 71.1 38.6 133.1 96 166.3V456c0 13.3 10.7 24 24 24s24-10.7 24-24V410c15.3 3.9 31.4 6 48 6c5.4 0 10.7-.2 16-.7V456c0 13.3 10.7 24 24 24s24-10.7 24-24V405.1c12.4-4.4 24.2-10 35.2-16.7zM448 200a24 24 0 1 1 0 48 24 24 0 1 1 0-48z"
        /> < title > { title } < / title > < / svg >
    }
}
