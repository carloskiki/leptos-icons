#[cfg(feature = "AiOutlinedStock")]
use leptos::*;
#[cfg(feature = "AiOutlinedStock")]
///This icon requires the feature `AiOutlinedStock` to be enabled.
#[component]
pub fn Stock(
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
        stroke_witdh = "0" style = style class = "icon" viewBox = "0 0 1024 1024" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M904 747H120c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8h784c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8zM165.7 621.8l39.7 39.5c3.1 3.1 8.2 3.1 11.3 0l234.7-233.9 97.6 97.3a32.11 32.11 0 0 0 45.2 0l264.2-263.2c3.1-3.1 3.1-8.2 0-11.3l-39.7-39.6a8.03 8.03 0 0 0-11.3 0l-235.7 235-97.7-97.3a32.11 32.11 0 0 0-45.2 0L165.7 610.5a7.94 7.94 0 0 0 0 11.3z"
        /> < title > { title } < / title > < / svg >
    }
}
