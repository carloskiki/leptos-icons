#[cfg(feature = "TiHeartHalfOutline")]
use leptos::*;
#[cfg(feature = "TiHeartHalfOutline")]
///This icon requires the feature `TiHeartHalfOutline` to be enabled.
#[component]
pub fn HeartHalfOutline(
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
        stroke_witdh = "0" style = style version = "1" width = "24" height = "24" viewBox
        = "0 0 24 24" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M2.2 9.4c0 1.3.2 3.3 2 5.1 1.6 1.6 6.9 5.2 7.1 5.4.2.1.4.2.6.2s.4-.1.6-.2c.2-.2 5.5-3.7 7.1-5.4 1.8-1.8 2-3.8 2-5.1 0-3-2.4-5.4-5.4-5.4-1.6 0-3.2.9-4.2 2.3-1-1.4-2.6-2.3-4.4-2.3-2.9 0-5.4 2.4-5.4 5.4zm9.8 1c.6 0 1-.4 1-1 0-1.9 1.5-3.4 3.4-3.4s3.4 1.5 3.4 3.4c0 1.1-.2 2.4-1.5 3.7-1.2 1.2-4.9 3.8-6.3 4.7v-7.4z"
        /> < title > { title } < / title > < / svg >
    }
}
