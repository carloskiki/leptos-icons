#[cfg(feature = "FaSolidWineGlassEmpty")]
use leptos::*;
#[cfg(feature = "FaSolidWineGlassEmpty")]
///This icon requires the feature `FaSolidWineGlassEmpty` to be enabled.
#[component]
pub fn WineGlassEmpty(
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
        stroke_witdh = "0" style = style viewBox = "0 0 320 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M47 0C30.4 0 16.5 12.8 15.1 29.3L1.1 197.8c-6 72 42.5 135.2 109.9 150.6V448H63c-17.7 0-32 14.3-32 32s14.3 32 32 32h80 80c17.7 0 32-14.3 32-32s-14.3-32-32-32H175V348.4c67.4-15.4 115.9-78.6 109.9-150.6l-14-168.4C269.5 12.8 255.6 0 239 0H47zM64.9 203.1L76.4 64H209.6l11.6 139.1C225 248.8 188.9 288 143 288s-82-39.2-78.1-84.9z"
        /> < title > { title } < / title > < / svg >
    }
}
