#[cfg(feature = "WiDirectionLeft")]
use leptos::*;
#[cfg(feature = "WiDirectionLeft")]
///This icon requires the feature `WiDirectionLeft` to be enabled.
#[component]
pub fn DirectionLeft(
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
        "M7.09,14.96c0,0.37,0.12,0.68,0.37,0.92l3.84,3.75c0.22,0.25,0.51,0.38,0.85,0.38c0.35,0,0.65-0.12,0.89-0.35&#xA;	s0.37-0.53,0.37-0.88s-0.12-0.65-0.37-0.89l-1.64-1.64h10.3c0.35,0,0.64-0.12,0.87-0.37s0.34-0.55,0.34-0.9s-0.11-0.65-0.34-0.9&#xA;	s-0.52-0.38-0.87-0.39H11.4l1.64-1.66c0.24-0.24,0.37-0.53,0.37-0.86c0-0.35-0.12-0.65-0.37-0.89S12.5,9.9,12.14,9.9&#xA;	c-0.32,0-0.61,0.14-0.85,0.41l-3.84,3.75C7.21,14.31,7.09,14.6,7.09,14.96z"
        /> < title > { title } < / title > < / svg >
    }
}
