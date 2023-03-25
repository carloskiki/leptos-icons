#[cfg(feature = "FaSolidBorderTopLeft")]
use leptos::*;
#[cfg(feature = "FaSolidBorderTopLeft")]
///This icon requires the feature `FaSolidBorderTopLeft` to be enabled.
#[component]
pub fn BorderTopLeft(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M0 448c0 17.7 14.3 32 32 32s32-14.3 32-32l0-336c0-8.8 7.2-16 16-16l336 0c17.7 0 32-14.3 32-32s-14.3-32-32-32L80 32C35.8 32 0 67.8 0 112L0 448zm160 0a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm192 0a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm-96 0a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm192 0a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zM416 288a32 32 0 1 0 0-64 32 32 0 1 0 0 64zm0 32a32 32 0 1 0 0 64 32 32 0 1 0 0-64zm0-128a32 32 0 1 0 0-64 32 32 0 1 0 0 64z"
        /> < title > { title } < / title > < / svg >
    }
}
