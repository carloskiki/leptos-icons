#[cfg(feature = "FaSolidPersonDotsFromLine")]
use leptos::*;
#[cfg(feature = "FaSolidPersonDotsFromLine")]
///This icon requires the feature `FaSolidPersonDotsFromLine` to be enabled.
#[component]
pub fn PersonDotsFromLine(
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
        stroke_witdh = "0" style = style viewBox = "0 0 576 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M288 176A88 88 0 1 0 288 0a88 88 0 1 0 0 176zM78.7 372.9c15-12.5 50-34.4 97.3-50.1V432H400V322.7c47.3 15.8 82.3 37.7 97.3 50.1c20.4 17 50.6 14.2 67.6-6.1s14.2-50.6-6.1-67.6c-12-10-30.1-22.5-53.2-35C497.2 278.4 481.7 288 464 288c-26.5 0-48-21.5-48-48c0-4.3 .6-8.4 1.6-12.4C379.1 215.9 335.3 208 288 208c-60.2 0-114.9 12.9-160 29.9c0 .7 0 1.4 0 2.1c0 26.5-21.5 48-48 48c-11.8 0-22.7-4.3-31-11.4c-13.1 8.1-23.7 15.9-31.7 22.5c-20.4 17-23.1 47.2-6.1 67.6s47.2 23.1 67.6 6.1zM24 464c-13.3 0-24 10.7-24 24s10.7 24 24 24H552c13.3 0 24-10.7 24-24s-10.7-24-24-24H24zM224 280a24 24 0 1 1 48 0 24 24 0 1 1 -48 0zm104 56a24 24 0 1 1 0 48 24 24 0 1 1 0-48zM96 240a16 16 0 1 0 -32 0 16 16 0 1 0 32 0zm368 16a16 16 0 1 0 0-32 16 16 0 1 0 0 32z"
        /> < title > { title } < / title > < / svg >
    }
}
