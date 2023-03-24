#[cfg(feature = "IoTextSharp")]
use leptos::*;
#[cfg(feature = "IoTextSharp")]
///This icon requires the feature `IoTextSharp` to be enabled.
#[component]
pub fn TextSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M404.42,170c-41.23,0-78.07,24.06-93.85,61.3L304,246.52l40.33,17.18,6.56-15.22c8.9-21,29.91-34.55,53.53-34.55,34.55,0,57.76,23.27,57.76,57.91v2.3c-22.12.59-48.65,2.05-72.27,4.84-54.52,6.43-87.06,36.23-87.06,79.72,0,23.16,8.72,44,24.56,58.59C342.28,431,362.55,438,384.51,438c30.86,0,57.5-7.33,77.67-22.64V438H506V271.84C506,212.83,463.28,170,404.42,170ZM384.51,395.07c-17.46,0-37.85-9.84-37.85-36.37,0-10.65,3.82-18.11,12.38-24.19,8.34-5.92,21.12-10.15,36-11.9,21.78-2.57,46.31-3.95,67-4.52C459.88,369.58,434.47,395.07,384.51,395.07Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M93.25,325.87h125.5L260.94,438H308L155,48,4,438H51.06ZM156,160.71,202.25,282h-92.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
