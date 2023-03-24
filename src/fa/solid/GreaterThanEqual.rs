#[cfg(feature = "FaSolidGreaterThanEqual")]
use leptos::*;
#[cfg(feature = "FaSolidGreaterThanEqual")]
///This icon requires the feature `FaSolidGreaterThanEqual` to be enabled.
#[component]
pub fn GreaterThanEqual(
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
        "M52.1 93.7C35.7 87.1 27.7 68.5 34.3 52.1s25.2-24.4 41.6-17.8l320 128C408 167.1 416 178.9 416 192s-8 24.9-20.1 29.7l-320 128c-16.4 6.6-35-1.4-41.6-17.8s1.4-35 17.8-41.6L297.8 192 52.1 93.7zM416 416c17.7 0 32 14.3 32 32s-14.3 32-32 32H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H416z"
        /> < title > { title } < / title > < / svg >
    }
}
