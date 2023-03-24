#[cfg(feature = "HiLgOutlineChevronLeft")]
use leptos::*;
#[cfg(feature = "HiLgOutlineChevronLeft")]
///This icon requires the feature `HiLgOutlineChevronLeft` to be enabled.
#[component]
pub fn ChevronLeft(
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
        "M15.75 19.5L8.25 12L15.75 4.5" stroke = "#0F172A" stroke - width = "1.5" stroke
        - linecap = "round" stroke - linejoin = "round" /> < title > { title } < / title
        > < / svg >
    }
}
