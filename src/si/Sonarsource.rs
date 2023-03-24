#[cfg(feature = "SiSonarsource")]
use leptos::*;
#[cfg(feature = "SiSonarsource")]
///This icon requires the feature `SiSonarsource` to be enabled.
#[component]
pub fn Sonarsource(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M15.71.4l-.467.737c3.476 2.14 6.284 5.28 7.955 8.89L24 9.694c-1.671-3.81-4.68-7.086-8.29-9.292zM8.49.87l-.333 1.069c6.952 2.006 12.434 7.62 14.039 14.44l1.069-.268C21.527 8.958 15.778 2.942 8.49.87zM0 2.607v1.338c10.964 0 19.922 8.824 19.922 19.654h1.337C21.26 12.034 11.7 2.607 0 2.607z"
        /> < title > { title } < / title > < / svg >
    }
}
