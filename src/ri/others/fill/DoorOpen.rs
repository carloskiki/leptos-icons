#[cfg(feature = "RiOthersFillDoorOpen")]
use leptos::*;
#[cfg(feature = "RiOthersFillDoorOpen")]
///This icon requires the feature `RiOthersFillDoorOpen` to be enabled.
#[component]
pub fn DoorOpen(
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
        "M2 21v-2h2V4.835c0-.484.346-.898.821-.984l9.472-1.722c.326-.06.638.157.697.483.007.035.01.07.01.107v1.28L19 4c.552 0 1 .448 1 1v14h2v2h-4V6h-3v15H2zm10-10h-2v2h2v-2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
