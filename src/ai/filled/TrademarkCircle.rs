#[cfg(feature = "AiFilledTrademarkCircle")]
use leptos::*;
#[cfg(feature = "AiFilledTrademarkCircle")]
///This icon requires the feature `AiFilledTrademarkCircle` to be enabled.
#[component]
pub fn TrademarkCircle(
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
        stroke_witdh = "0" style = style class = "icon" viewBox = "0 0 1024 1024" width =
        { size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg"
        d =
        "M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm164.7 660.2c-1.1.5-2.3.8-3.5.8h-62c-3.1 0-5.9-1.8-7.2-4.6l-74.6-159.2h-88.7V717c0 4.4-3.6 8-8 8H378c-4.4 0-8-3.6-8-8V307c0-4.4 3.6-8 8-8h155.6c98.8 0 144.2 59.9 144.2 131.1 0 70.2-43.6 106.4-78.4 119.2l80.8 164.2c2.1 3.9.4 8.7-3.5 10.7zM523.9 357h-83.4v148H522c53 0 82.8-25.6 82.8-72.4 0-50.3-32.9-75.6-80.9-75.6z"
        /> < title > { title } < / title > < / svg >
    }
}
