#[cfg(feature = "AiTwotoneUpSquare")]
use leptos::*;
#[cfg(feature = "AiTwotoneUpSquare")]
///This icon requires the feature `AiTwotoneUpSquare` to be enabled.
#[component]
pub fn UpSquare(
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
        stroke_witdh = "0" style = style viewBox = "0 0 1024 1024" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" fill = "#333" d =
        "M880 112H144c-17.7 0-32 14.3-32 32v736c0 17.7 14.3 32 32 32h736c17.7 0 32-14.3 32-32V144c0-17.7-14.3-32-32-32zm-40 728H184V184h656v656z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill = "#E6E6E6" d =
        "M184 840h656V184H184v656zm143.5-228.7l178-246c3.2-4.4 9.7-4.4 12.9 0l178 246c3.9 5.3.1 12.7-6.4 12.7h-46.9c-10.2 0-19.9-4.9-25.9-13.2L512 465.4 406.8 610.8c-6 8.3-15.6 13.2-25.9 13.2H334c-6.5 0-10.3-7.4-6.5-12.7z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill = "#333" d =
        "M334 624h46.9c10.3 0 19.9-4.9 25.9-13.2L512 465.4l105.2 145.4c6 8.3 15.7 13.2 25.9 13.2H690c6.5 0 10.3-7.4 6.4-12.7l-178-246a7.95 7.95 0 0 0-12.9 0l-178 246c-3.8 5.3 0 12.7 6.5 12.7z"
        /> < title > { title } < / title > < / svg >
    }
}
