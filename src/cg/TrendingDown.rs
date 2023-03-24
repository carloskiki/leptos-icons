#[cfg(feature = "CgTrendingDown")]
use leptos::*;
#[cfg(feature = "CgTrendingDown")]
///This icon requires the feature `CgTrendingDown` to be enabled.
#[component]
pub fn TrendingDown(
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
        "M1.85104 8.10628L0.436829 9.52049L7.5079 16.5916L13.8719 10.2276L18.1145 14.4702L16.3721 16.2126L23.0642 18.0058L21.2711 11.3136L19.5287 13.056L13.8719 7.39917L7.5079 13.7631L1.85104 8.10628Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
