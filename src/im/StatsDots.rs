#[cfg(feature = "ImStatsDots")]
use leptos::*;
#[cfg(feature = "ImStatsDots")]
///This icon requires the feature `ImStatsDots` to be enabled.
#[component]
pub fn StatsDots(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M2 14h14v2h-16v-16h2zM4.5 13c-0.828 0-1.5-0.672-1.5-1.5s0.672-1.5 1.5-1.5c0.044 0 0.088 0.002 0.131 0.006l1.612-2.687c-0.154-0.235-0.243-0.517-0.243-0.819 0-0.828 0.672-1.5 1.5-1.5s1.5 0.672 1.5 1.5c0 0.302-0.090 0.583-0.243 0.819l1.612 2.687c0.043-0.004 0.087-0.006 0.131-0.006 0.033 0 0.066 0.001 0.099 0.004l2.662-4.658c-0.165-0.241-0.261-0.532-0.261-0.845 0-0.828 0.672-1.5 1.5-1.5s1.5 0.672 1.5 1.5c0 0.828-0.672 1.5-1.5 1.5-0.033 0-0.066-0.001-0.099-0.004l-2.662 4.658c0.165 0.241 0.261 0.532 0.261 0.845 0 0.828-0.672 1.5-1.5 1.5s-1.5-0.672-1.5-1.5c0-0.302 0.090-0.583 0.243-0.819l-1.612-2.687c-0.043 0.004-0.087 0.006-0.131 0.006s-0.088-0.002-0.131-0.006l-1.612 2.687c0.154 0.235 0.243 0.517 0.243 0.819 0 0.828-0.672 1.5-1.5 1.5z"
        /> < title > { title } < / title > < / svg >
    }
}
