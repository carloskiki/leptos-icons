#[cfg(feature = "ImMusic")]
use leptos::*;
#[cfg(feature = "ImMusic")]
///This icon requires the feature `ImMusic` to be enabled.
#[component]
pub fn Music(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M15 0h1v11.5c0 1.381-1.567 2.5-3.5 2.5s-3.5-1.119-3.5-2.5c0-1.381 1.567-2.5 3.5-2.5 0.979 0 1.865 0.287 2.5 0.751v-5.751l-8 1.778v7.722c0 1.381-1.567 2.5-3.5 2.5s-3.5-1.119-3.5-2.5c0-1.381 1.567-2.5 3.5-2.5 0.979 0 1.865 0.287 2.5 0.751v-9.751l9-2z"
        /> < title > { title } < / title > < / svg >
    }
}
