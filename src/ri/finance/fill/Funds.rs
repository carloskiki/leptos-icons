#[cfg(feature = "RiFinanceFillFunds")]
use leptos::*;
#[cfg(feature = "RiFinanceFillFunds")]
///This icon requires the feature `RiFinanceFillFunds` to be enabled.
#[component]
pub fn Funds(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M3.897 17.86l3.91-3.91 2.829 2.828 4.571-4.57L17 14V9h-5l1.793 1.793-3.157 3.157-2.828-2.829-4.946 4.946A9.965 9.965 0 0 1 2 12C2 6.477 6.477 2 12 2s10 4.477 10 10-4.477 10-10 10a9.987 9.987 0 0 1-8.103-4.14z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
