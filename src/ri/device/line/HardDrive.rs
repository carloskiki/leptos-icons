#[cfg(feature = "RiDeviceLineHardDrive")]
use leptos::*;
#[cfg(feature = "RiDeviceLineHardDrive")]
///This icon requires the feature `RiDeviceLineHardDrive` to be enabled.
#[component]
pub fn HardDrive(
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
        "none" d = "M0 0h24v24H0z" />< path d =
        "M5 10.938A8.004 8.004 0 0 0 11.938 4H5v6.938zm0 2.013V20h14V4h-5.05A10.003 10.003 0 0 1 5 12.95zM4 2h16a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1zm11 14h2v2h-2v-2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
