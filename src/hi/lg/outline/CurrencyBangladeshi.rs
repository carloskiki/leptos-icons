#[cfg(feature = "HiLgOutlineCurrencyBangladeshi")]
use leptos::*;
#[cfg(feature = "HiLgOutlineCurrencyBangladeshi")]
///This icon requires the feature `HiLgOutlineCurrencyBangladeshi` to be enabled.
#[component]
pub fn CurrencyBangladeshi(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.25 7.49997L8.66459 7.29267C9.16327 7.04333 9.75 7.40596 9.75 7.96349V10.5M9.75 10.5H15.75M9.75 10.5H8.25M9.75 10.5V15.9383C9.75 16.2921 9.91144 16.6351 10.2229 16.803C10.7518 17.0882 11.357 17.25 12 17.25C13.8142 17.25 15.3275 15.9617 15.675 14.25C15.7579 13.8414 15.412 13.5 14.995 13.5H14.25M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
