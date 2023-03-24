#[cfg(feature = "FaSolidFaceRollingEyes")]
use leptos::*;
#[cfg(feature = "FaSolidFaceRollingEyes")]
///This icon requires the feature `FaSolidFaceRollingEyes` to be enabled.
#[component]
pub fn FaceRollingEyes(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M256 512A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM192 368H320c8.8 0 16 7.2 16 16s-7.2 16-16 16H192c-8.8 0-16-7.2-16-16s7.2-16 16-16zm32-144c0 35.3-28.7 64-64 64s-64-28.7-64-64c0-26 15.5-48.4 37.8-58.4c-3.7 5.2-5.8 11.6-5.8 18.4c0 17.7 14.3 32 32 32s32-14.3 32-32c0-6.9-2.2-13.2-5.8-18.4C208.5 175.6 224 198 224 224zm128 64c-35.3 0-64-28.7-64-64c0-26 15.5-48.4 37.8-58.4c-3.7 5.2-5.8 11.6-5.8 18.4c0 17.7 14.3 32 32 32s32-14.3 32-32c0-6.9-2.2-13.2-5.8-18.4C400.5 175.6 416 198 416 224c0 35.3-28.7 64-64 64z"
        /> < title > { title } < / title > < / svg >
    }
}
