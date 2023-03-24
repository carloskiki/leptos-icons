#[cfg(feature = "FaSolidMercury")]
use leptos::*;
#[cfg(feature = "FaSolidMercury")]
///This icon requires the feature `FaSolidMercury` to be enabled.
#[component]
pub fn Mercury(
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
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M56.1 7C69.8-4 90-1.8 101 12c17.6 22 44.7 36 75 36s57.3-14 75-36c11.1-13.8 31.2-16 45-5s16 31.2 5 45c-7.8 9.7-16.6 18.4-26.4 26.1C321.3 109.7 352 163.3 352 224c0 89.1-66.2 162.7-152 174.4V424h32c13.3 0 24 10.7 24 24s-10.7 24-24 24H200v16c0 13.3-10.7 24-24 24s-24-10.7-24-24V472H120c-13.3 0-24-10.7-24-24s10.7-24 24-24h32V398.4C66.2 386.7 0 313.1 0 224C0 163.3 30.7 109.7 77.5 78.1C67.7 70.5 58.9 61.7 51.1 52c-11.1-13.8-8.8-33.9 5-45zM64 224a112 112 0 1 0 224 0A112 112 0 1 0 64 224z"
        /> < title > { title } < / title > < / svg >
    }
}
