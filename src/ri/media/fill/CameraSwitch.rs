#[cfg(feature = "RiMediaFillCameraSwitch")]
use leptos::*;
#[cfg(feature = "RiMediaFillCameraSwitch")]
///This icon requires the feature `RiMediaFillCameraSwitch` to be enabled.
#[component]
pub fn CameraSwitch(
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
        "M9 3h6l2 2h4a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V6a1 1 0 0 1 1-1h4l2-2zm5.684 15.368l-.895-1.79A4 4 0 0 1 8 13h2.001L7.839 8.677a6 6 0 0 0 6.845 9.69zM9.316 7.632l.895 1.79A4 4 0 0 1 16 13h-2.001l2.161 4.323a6 6 0 0 0-6.845-9.69z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
