#[cfg(feature = "FaSolidP")]
use leptos::*;
#[cfg(feature = "FaSolidP")]
///This icon requires the feature `FaSolidP` to be enabled.
#[component]
pub fn P(
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
        "M0 96C0 60.7 28.7 32 64 32h96c88.4 0 160 71.6 160 160s-71.6 160-160 160H64v96c0 17.7-14.3 32-32 32s-32-14.3-32-32V320 96zM64 288h96c53 0 96-43 96-96s-43-96-96-96H64V288z"
        /> < title > { title } < / title > < / svg >
    }
}
