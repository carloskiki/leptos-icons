#[cfg(feature = "SiKaniko")]
use leptos::*;
#[cfg(feature = "SiKaniko")]
///This icon requires the feature `SiKaniko` to be enabled.
#[component]
pub fn Kaniko(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M2.783 0h18.434c1.352 0 2.478.963 2.73 2.24a17.127 17.127 0 0 1-3.2 4.42 16.918 16.918 0 0 1-8.399 4.605V3.304h-.696V11.4c-.976.169-1.965.253-2.956.252v.696c1.011 0 1.998.086 2.956.252v8.096h.696v-7.961a16.918 16.918 0 0 1 8.399 4.605 17.127 17.127 0 0 1 3.2 4.42 2.783 2.783 0 0 1-2.73 2.24H2.783A2.783 2.783 0 0 1 0 21.217V2.783A2.783 2.783 0 0 1 2.783 0Zm18.456 7.152A17.712 17.712 0 0 0 24 3.572v16.856a17.712 17.712 0 0 0-2.761-3.58 17.802 17.802 0 0 0-8.891-4.815v-.066a17.802 17.802 0 0 0 8.891-4.815Z"
        /> < title > { title } < / title > < / svg >
    }
}
