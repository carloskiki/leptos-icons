#[cfg(feature = "RiOthersLineDoorClosed")]
use leptos::*;
#[cfg(feature = "RiOthersLineDoorClosed")]
///This icon requires the feature `RiOthersLineDoorClosed` to be enabled.
#[component]
pub fn DoorClosed(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0H24V24H0z" />< path d =
        "M3 21v-2h2V4c0-.552.448-1 1-1h12c.552 0 1 .448 1 1v15h2v2H3zM17 5H7v14h10V5zm-2 6v2h-2v-2h2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
