#[cfg(feature = "RiFinanceFillMoneyEuroBox")]
use leptos::*;
#[cfg(feature = "RiFinanceFillMoneyEuroBox")]
///This icon requires the feature `RiFinanceFillMoneyEuroBox` to be enabled.
#[component]
pub fn MoneyEuroBox(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path d =
        "M3 3h18a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1zm7.05 8a2.5 2.5 0 0 1 4.064-1.41l1.701-1.133A4.5 4.5 0 0 0 8.028 11H7v2h1.027a4.5 4.5 0 0 0 7.788 2.543l-1.701-1.134A2.5 2.5 0 0 1 10.05 13l4.95.001v-2h-4.95z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
