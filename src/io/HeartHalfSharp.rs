#[cfg(feature = "IoHeartHalfSharp")]
use leptos::*;
#[cfg(feature = "IoHeartHalfSharp")]
///This icon requires the feature `IoHeartHalfSharp` to be enabled.
#[component]
pub fn HeartHalfSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M352.92,64c-48.11,0-80.1,28-96.92,48.21C239.18,92,207.19,64,159.08,64,98.46,64,48.63,114.54,48,176.65c-.56,55.68,19.71,107,62,157,40.12,47.46,94.25,79.75,137,108.32l9,6,9-6c42.78-28.57,96.91-60.86,137-108.32,42.25-50,62.52-101.35,62-157C463.37,114.54,413.54,64,352.92,64Zm24.67,249c-31.78,37.6-74.68,65.75-112.52,90.59l-9.07,6V162.23l24.59-29.54C294.53,116,318.38,96,352.92,96c43.15,0,78.62,36.32,79.07,81a178.63,178.63,0,0,1-12.69,68.59C410.27,268.43,396.63,290.5,377.59,313Z"
        /> < title > { title } < / title > < / svg >
    }
}
