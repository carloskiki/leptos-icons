#[cfg(feature = "IoRecordingSharp")]
use leptos::*;
#[cfg(feature = "IoRecordingSharp")]
///This icon requires the feature `IoRecordingSharp` to be enabled.
#[component]
pub fn RecordingSharp(
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
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M384,138a117.93,117.93,0,0,0-91.84,192H219.84A118,118,0,1,0,128,374H384a118,118,0,0,0,0-236ZM54,256a74,74,0,1,1,74,74A74.09,74.09,0,0,1,54,256Zm330,74a74,74,0,1,1,74-74A74.09,74.09,0,0,1,384,330Z"
        /> < title > { title } < / title > < / svg >
    }
}
