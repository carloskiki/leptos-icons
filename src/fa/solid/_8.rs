#[cfg(feature = "FaSolid8")]
use leptos::*;
#[cfg(feature = "FaSolid8")]
///This icon requires the feature `FaSolid8` to be enabled.
#[component]
pub fn _8(
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
        "M304 160c0-70.7-57.3-128-128-128H144C73.3 32 16 89.3 16 160c0 34.6 13.7 66 36 89C20.5 272.3 0 309.8 0 352c0 70.7 57.3 128 128 128h64c70.7 0 128-57.3 128-128c0-42.2-20.5-79.7-52-103c22.3-23 36-54.4 36-89zM176.1 288H192c35.3 0 64 28.7 64 64s-28.7 64-64 64H128c-35.3 0-64-28.7-64-64s28.7-64 64-64h15.9c0 0 .1 0 .1 0h32c0 0 .1 0 .1 0zm0-64c0 0 0 0 0 0H144c0 0 0 0 0 0c-35.3 0-64-28.7-64-64c0-35.3 28.7-64 64-64h32c35.3 0 64 28.7 64 64c0 35.3-28.6 64-64 64z"
        /> < title > { title } < / title > < / svg >
    }
}
