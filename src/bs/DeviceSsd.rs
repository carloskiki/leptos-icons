#[cfg(feature = "BsDeviceSsd")]
use leptos::*;
#[cfg(feature = "BsDeviceSsd")]
///This icon requires the feature `BsDeviceSsd` to be enabled.
#[component]
pub fn DeviceSsd(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-device-ssd" viewBox = "0 0 16 16" width = { size.clone() } height
        = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M4.75 4a.75.75 0 0 0-.75.75v3.5c0 .414.336.75.75.75h6.5a.75.75 0 0 0 .75-.75v-3.5a.75.75 0 0 0-.75-.75h-6.5ZM5 8V5h6v3H5Zm0-5.5a.5.5 0 1 1-1 0 .5.5 0 0 1 1 0Zm7 0a.5.5 0 1 1-1 0 .5.5 0 0 1 1 0ZM4.5 11a.5.5 0 1 0 0-1 .5.5 0 0 0 0 1Zm7 0a.5.5 0 1 0 0-1 .5.5 0 0 0 0 1Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2 2a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V2Zm11 12V2a1 1 0 0 0-1-1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1v-2a1 1 0 0 1 1-1h6a1 1 0 0 1 1 1v2a1 1 0 0 0 1-1Zm-7.25 1v-2H5v2h.75Zm1.75 0v-2h-.75v2h.75Zm1.75 0v-2H8.5v2h.75ZM11 13h-.75v2H11v-2Z"
        /> < title > { title } < / title > < / svg >
    }
}
