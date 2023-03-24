#[cfg(feature = "FaSolidArrowsTurnToDots")]
use leptos::*;
#[cfg(feature = "FaSolidArrowsTurnToDots")]
///This icon requires the feature `FaSolidArrowsTurnToDots` to be enabled.
#[component]
pub fn ArrowsTurnToDots(
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
        "M249.4 25.4c12.5-12.5 32.8-12.5 45.3 0s12.5 32.8 0 45.3L269.3 96 416 96c53 0 96 43 96 96v32c0 17.7-14.3 32-32 32s-32-14.3-32-32V192c0-17.7-14.3-32-32-32l-146.7 0 25.4 25.4c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0l-80-80c-12.5-12.5-12.5-32.8 0-45.3l80-80zm13.3 256l80 80c12.5 12.5 12.5 32.8 0 45.3l-80 80c-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3L242.7 416 96 416c-17.7 0-32 14.3-32 32v32c0 17.7-14.3 32-32 32s-32-14.3-32-32V448c0-53 43-96 96-96l146.7 0-25.4-25.4c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0zM384 384a64 64 0 1 1 128 0 64 64 0 1 1 -128 0zM64 192A64 64 0 1 1 64 64a64 64 0 1 1 0 128z"
        /> < title > { title } < / title > < / svg >
    }
}
