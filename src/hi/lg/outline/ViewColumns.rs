#[cfg(feature = "HiLgOutlineViewColumns")]
use leptos::*;
#[cfg(feature = "HiLgOutlineViewColumns")]
///This icon requires the feature `HiLgOutlineViewColumns` to be enabled.
#[component]
pub fn ViewColumns(
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
        "M9 4.5V19.5M15 4.5V19.5M4.125 19.5H19.875C20.4963 19.5 21 18.9963 21 18.375V5.625C21 5.00368 20.4963 4.5 19.875 4.5H4.125C3.50368 4.5 3 5.00368 3 5.625V18.375C3 18.9963 3.50368 19.5 4.125 19.5Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
