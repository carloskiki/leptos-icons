#[cfg(feature = "BiSolidVolumeMute")]
use leptos::*;
#[cfg(feature = "BiSolidVolumeMute")]
///This icon requires the feature `BiSolidVolumeMute` to be enabled.
#[component]
pub fn VolumeMute(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "m7.727 6.313-4.02-4.02-1.414 1.414 18 18 1.414-1.414-2.02-2.02A9.578 9.578 0 0 0 21.999 12c0-4.091-2.472-7.453-5.999-9v2c2.387 1.386 3.999 4.047 3.999 7a8.13 8.13 0 0 1-1.671 4.914l-1.286-1.286C17.644 14.536 18 13.19 18 12c0-1.771-.775-3.9-2-5v7.586l-2-2V2.132L7.727 6.313zM4 17h2.697L14 21.868v-3.747L3.102 7.223A1.995 1.995 0 0 0 2 9v6c0 1.103.897 2 2 2z"
        /> < title > { title } < / title > < / svg >
    }
}
