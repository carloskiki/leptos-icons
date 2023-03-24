#[cfg(feature = "FaSolidTrademark")]
use leptos::*;
#[cfg(feature = "FaSolidTrademark")]
///This icon requires the feature `FaSolidTrademark` to be enabled.
#[component]
pub fn Trademark(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M345.6 108.8c-8.3-11-22.7-15.5-35.7-11.2S288 114.2 288 128V384c0 17.7 14.3 32 32 32s32-14.3 32-32V224l86.4 115.2c6 8.1 15.5 12.8 25.6 12.8s19.6-4.7 25.6-12.8L576 224V384c0 17.7 14.3 32 32 32s32-14.3 32-32V128c0-13.8-8.8-26-21.9-30.4s-27.5 .1-35.7 11.2L464 266.7 345.6 108.8zM0 128c0 17.7 14.3 32 32 32H96V384c0 17.7 14.3 32 32 32s32-14.3 32-32V160h64c17.7 0 32-14.3 32-32s-14.3-32-32-32H32C14.3 96 0 110.3 0 128z"
        /> < title > { title } < / title > < / svg >
    }
}
