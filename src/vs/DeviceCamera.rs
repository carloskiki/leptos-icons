#[cfg(feature = "VsDeviceCamera")]
use leptos::*;
#[cfg(feature = "VsDeviceCamera")]
///This icon requires the feature `VsDeviceCamera` to be enabled.
#[component]
pub fn DeviceCamera(
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
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M10.707 3H14.5l.5.5v9l-.5.5h-13l-.5-.5v-9l.5-.5h3.793l.853-.854L6.5 2h3l.354.146.853.854zM2 12h12V4h-3.5l-.354-.146L9.293 3H6.707l-.853.854L5.5 4H2v8zm1.5-7a.5.5 0 1 0 0 1 .5.5 0 0 0 0-1zM8 6a2 2 0 1 1 0 4 2 2 0 0 1 0-4zm0-1a3 3 0 1 0 0 6 3 3 0 0 0 0-6z"
        /> < title > { title } < / title > < / svg >
    }
}
