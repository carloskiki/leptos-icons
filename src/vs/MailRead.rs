#[cfg(feature = "VsMailRead")]
use leptos::*;
#[cfg(feature = "VsMailRead")]
///This icon requires the feature `VsMailRead` to be enabled.
#[component]
pub fn MailRead(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M8.25 1.57h-.51L1 5.56v7.94l.5.5h13l.5-.5V5.56L8.25 1.57zM8 2.58l5.63 3.32-1.37 1.59H3.74L2.43 5.9 8 2.58zM14 13H2V6.92L3.11 8.3l.39.19h9l.39-.19L14 6.92V13z"
        /> < title > { title } < / title > < / svg >
    }
}
