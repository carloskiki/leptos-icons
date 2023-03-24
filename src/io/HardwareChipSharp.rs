#[cfg(feature = "IoHardwareChipSharp")]
use leptos::*;
#[cfg(feature = "IoHardwareChipSharp")]
///This icon requires the feature `IoHardwareChipSharp` to be enabled.
#[component]
pub fn HardwareChipSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < rect xmlns =
        "http://www.w3.org/2000/svg" x = "160" y = "160" width = "192" height = "192" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M480,198V154H448V88a24,24,0,0,0-24-24H358V32H314V64H278V32H234V64H198V32H154V64H88A24,24,0,0,0,64,88v66H32v44H64v36H32v44H64v36H32v44H64v66a24,24,0,0,0,24,24h66v32h44V448h36v32h44V448h36v32h44V448h66a24,24,0,0,0,24-24V358h32V314H448V278h32V234H448V198ZM128,128H384V384H128Z"
        /> < title > { title } < / title > < / svg >
    }
}
