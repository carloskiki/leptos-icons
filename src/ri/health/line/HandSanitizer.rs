#[cfg(feature = "RiHealthLineHandSanitizer")]
use leptos::*;
#[cfg(feature = "RiHealthLineHandSanitizer")]
///This icon requires the feature `RiHealthLineHandSanitizer` to be enabled.
#[component]
pub fn HandSanitizer(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M17 2v2l-4-.001V6h3v2c2.21 0 4 1.79 4 4v8c0 1.105-.895 2-2 2H6c-1.105 0-2-.895-2-2v-8c0-2.21 1.79-4 4-4V6h3V3.999L7.5 4c-.63 0-1.37.49-2.2 1.6L3.7 4.4C4.87 2.84 6.13 2 7.5 2H17zm-1 8H8c-1.105 0-2 .895-2 2v8h12v-8c0-1.105-.895-2-2-2zm-3 2v2h2v2h-2.001L13 18h-2l-.001-2H9v-2h2v-2h2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
