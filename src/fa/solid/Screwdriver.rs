#[cfg(feature = "FaSolidScrewdriver")]
use leptos::*;
#[cfg(feature = "FaSolidScrewdriver")]
///This icon requires the feature `FaSolidScrewdriver` to be enabled.
#[component]
pub fn Screwdriver(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M465 7c-8.5-8.5-22-9.4-31.6-2.1l-104 80c-5.9 4.5-9.4 11.6-9.4 19v54.1l-85.6 85.6c6.7 4.2 13 9.3 18.8 15.1s10.9 12.2 15.1 18.8L353.9 192H408c7.5 0 14.5-3.5 19-9.4l80-104c7.4-9.6 6.5-23.1-2.1-31.6L465 7zM121.4 281.4l-112 112c-12.5 12.5-12.5 32.8 0 45.3l64 64c12.5 12.5 32.8 12.5 45.3 0l112-112c30.2-30.2 30.2-79.1 0-109.3s-79.1-30.2-109.3 0z"
        /> < title > { title } < / title > < / svg >
    }
}
