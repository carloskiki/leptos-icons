#[cfg(feature = "FaSolidPesetaSign")]
use leptos::*;
#[cfg(feature = "FaSolidPesetaSign")]
///This icon requires the feature `FaSolidPesetaSign` to be enabled.
#[component]
pub fn PesetaSign(
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
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M64 32C46.3 32 32 46.3 32 64v96c-17.7 0-32 14.3-32 32s14.3 32 32 32l0 96V448c0 17.7 14.3 32 32 32s32-14.3 32-32V352h96c77.4 0 142-55 156.8-128H352c17.7 0 32-14.3 32-32s-14.3-32-32-32h-3.2C334 87 269.4 32 192 32H64zM282.5 160H96V96h96c41.8 0 77.4 26.7 90.5 64zM96 224H282.5c-13.2 37.3-48.7 64-90.5 64H96V224z"
        /> < title > { title } < / title > < / svg >
    }
}
