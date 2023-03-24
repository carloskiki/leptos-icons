#[cfg(feature = "AiFilledDownSquare")]
use leptos::*;
#[cfg(feature = "AiFilledDownSquare")]
///This icon requires the feature `AiFilledDownSquare` to be enabled.
#[component]
pub fn DownSquare(
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
        "M880 112H144c-17.7 0-32 14.3-32 32v736c0 17.7 14.3 32 32 32h736c17.7 0 32-14.3 32-32V144c0-17.7-14.3-32-32-32zM696.5 412.7l-178 246a7.95 7.95 0 0 1-12.9 0l-178-246c-3.8-5.3 0-12.7 6.5-12.7H381c10.2 0 19.9 4.9 25.9 13.2L512 558.6l105.2-145.4c6-8.3 15.6-13.2 25.9-13.2H690c6.5 0 10.3 7.4 6.5 12.7z"
        /> < title > { title } < / title > < / svg >
    }
}
