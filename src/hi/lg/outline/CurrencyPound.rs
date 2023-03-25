#[cfg(feature = "HiLgOutlineCurrencyPound")]
use leptos::*;
#[cfg(feature = "HiLgOutlineCurrencyPound")]
///This icon requires the feature `HiLgOutlineCurrencyPound` to be enabled.
#[component]
pub fn CurrencyPound(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.1213 7.62877C12.9497 6.45719 11.0503 6.45719 9.87868 7.62877C9.37424 8.13321 9.08699 8.7726 9.01694 9.43073C8.9944 9.64251 9.01512 9.85582 9.04524 10.0667L9.5512 13.6084C9.68065 14.5146 9.5307 15.4386 9.12135 16.2573L9 16.5L10.5385 15.9872C11.0003 15.8332 11.4997 15.8332 11.9615 15.9872L12.6158 16.2053C13.182 16.394 13.7999 16.3501 14.3336 16.0832L15 15.75M8.25 12H12M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
