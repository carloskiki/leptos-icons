#[cfg(feature = "FaSolidBridgeCircleCheck")]
use leptos::*;
#[cfg(feature = "FaSolidBridgeCircleCheck")]
///This icon requires the feature `FaSolidBridgeCircleCheck` to be enabled.
#[component]
pub fn BridgeCircleCheck(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M64 32C46.3 32 32 46.3 32 64s14.3 32 32 32h40v64H32V288c53 0 96 43 96 96v64c0 17.7 14.3 32 32 32h32c17.7 0 32-14.3 32-32V384c0-53 43-96 96-96c6.3 0 12.4 .6 18.3 1.7C367.1 231.8 426.9 192 496 192c42.5 0 81.6 15.1 112 40.2V160H536V96h40c17.7 0 32-14.3 32-32s-14.3-32-32-32H64zM488 96v64H408V96h80zM360 96v64H280V96h80zM232 96v64H152V96h80zM640 368a144 144 0 1 0 -288 0 144 144 0 1 0 288 0zm-76.7-43.3c6.2 6.2 6.2 16.4 0 22.6l-72 72c-6.2 6.2-16.4 6.2-22.6 0l-40-40c-6.2-6.2-6.2-16.4 0-22.6s16.4-6.2 22.6 0L480 385.4l60.7-60.7c6.2-6.2 16.4-6.2 22.6 0z"
        /> < title > { title } < / title > < / svg >
    }
}
