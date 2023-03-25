#[cfg(feature = "IoHeartSharp")]
use leptos::*;
#[cfg(feature = "IoHeartSharp")]
///This icon requires the feature `IoHeartSharp` to be enabled.
#[component]
pub fn HeartSharp(
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
        "M256,448l-9-6c-42.78-28.57-96.91-60.86-137-108.32-42.25-50-62.52-101.35-62-157C48.63,114.54,98.46,64,159.08,64c48.11,0,80.1,28,96.92,48.21C272.82,92,304.81,64,352.92,64,413.54,64,463.37,114.54,464,176.65c.56,55.68-19.71,107-62,157C361.91,381.14,307.78,413.43,265,442Z"
        /> < title > { title } < / title > < / svg >
    }
}
