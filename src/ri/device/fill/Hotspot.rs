#[cfg(feature = "RiDeviceFillHotspot")]
use leptos::*;
#[cfg(feature = "RiDeviceFillHotspot")]
///This icon requires the feature `RiDeviceFillHotspot` to be enabled.
#[component]
pub fn Hotspot(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M11 2v9h7v10a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1h6zm2 5a2 2 0 0 1 2 2h-2V7zm0-3a5 5 0 0 1 5 5h-2a3 3 0 0 0-3-3V4zm0-3a8 8 0 0 1 8 8h-2a6 6 0 0 0-6-6V1z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
