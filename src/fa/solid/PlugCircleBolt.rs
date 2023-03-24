#[cfg(feature = "FaSolidPlugCircleBolt")]
use leptos::*;
#[cfg(feature = "FaSolidPlugCircleBolt")]
///This icon requires the feature `FaSolidPlugCircleBolt` to be enabled.
#[component]
pub fn PlugCircleBolt(
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
        "M96 0C78.3 0 64 14.3 64 32v96h64V32c0-17.7-14.3-32-32-32zM288 0c-17.7 0-32 14.3-32 32v96h64V32c0-17.7-14.3-32-32-32zM32 160c-17.7 0-32 14.3-32 32s14.3 32 32 32v32c0 77.4 55 142 128 156.8V480c0 17.7 14.3 32 32 32s32-14.3 32-32V412.8c12.3-2.5 24.1-6.4 35.1-11.5c-2.1-10.8-3.1-21.9-3.1-33.3c0-80.3 53.8-148 127.3-169.2c.5-2.2 .7-4.5 .7-6.8c0-17.7-14.3-32-32-32H32zM432 512a144 144 0 1 0 0-288 144 144 0 1 0 0 288zm47.9-225c4.3 3.7 5.4 9.9 2.6 14.9L452.4 356H488c5.2 0 9.8 3.3 11.4 8.2s-.1 10.3-4.2 13.4l-96 72c-4.5 3.4-10.8 3.2-15.1-.6s-5.4-9.9-2.6-14.9L411.6 380H376c-5.2 0-9.8-3.3-11.4-8.2s.1-10.3 4.2-13.4l96-72c4.5-3.4 10.8-3.2 15.1 .6z"
        /> < title > { title } < / title > < / svg >
    }
}
