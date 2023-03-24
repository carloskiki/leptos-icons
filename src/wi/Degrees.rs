#[cfg(feature = "WiDegrees")]
use leptos::*;
#[cfg(feature = "WiDegrees")]
///This icon requires the feature `WiDegrees` to be enabled.
#[component]
pub fn Degrees(
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
        stroke_witdh = "0" style = style version = "1.1" id = "Layer_1" x = "0px" y =
        "0px" viewBox = "0 0 30 30" style = "enable-background:new 0 0 30 30;" space =
        "preserve" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" d =
        "M13.19,9.21c0-0.5,0.18-0.93,0.53-1.28c0.36-0.36,0.78-0.53,1.28-0.53c0.49,0,0.92,0.18,1.27,0.53&#xA;	c0.35,0.36,0.53,0.78,0.53,1.28s-0.18,0.93-0.53,1.29c-0.35,0.36-0.78,0.54-1.27,0.54c-0.49,0-0.92-0.18-1.28-0.54&#xA;	S13.19,9.71,13.19,9.21z M14.07,9.21c0,0.26,0.09,0.48,0.27,0.67c0.19,0.19,0.41,0.28,0.67,0.28c0.26,0,0.48-0.09,0.67-0.28&#xA;	s0.28-0.41,0.28-0.67c0-0.26-0.09-0.48-0.28-0.66c-0.19-0.18-0.41-0.28-0.67-0.28c-0.26,0-0.48,0.09-0.67,0.27&#xA;	C14.16,8.72,14.07,8.94,14.07,9.21z"
        /> < title > { title } < / title > < / svg >
    }
}
