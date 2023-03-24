#[cfg(feature = "FaBrandsSistrix")]
use leptos::*;
#[cfg(feature = "FaBrandsSistrix")]
///This icon requires the feature `FaBrandsSistrix` to be enabled.
#[component]
pub fn Sistrix(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M448 449L301.2 300.2c20-27.9 31.9-62.2 31.9-99.2 0-93.1-74.7-168.9-166.5-168.9C74.7 32 0 107.8 0 200.9s74.7 168.9 166.5 168.9c39.8 0 76.3-14.2 105-37.9l146 148.1 30.5-31zM166.5 330.8c-70.6 0-128.1-58.3-128.1-129.9S95.9 71 166.5 71s128.1 58.3 128.1 129.9-57.4 129.9-128.1 129.9z"
        /> < title > { title } < / title > < / svg >
    }
}
