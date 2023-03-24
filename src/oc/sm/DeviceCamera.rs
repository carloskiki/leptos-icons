#[cfg(feature = "OcSmDeviceCamera")]
use leptos::*;
#[cfg(feature = "OcSmDeviceCamera")]
///This icon requires the feature `OcSmDeviceCamera` to be enabled.
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M15 3c.55 0 1 .45 1 1v9c0 .55-.45 1-1 1H1c-.55 0-1-.45-1-1V4c0-.55.45-1 1-1 0-.55.45-1 1-1h4c.55 0 1 .45 1 1Zm-4.5 9c1.94 0 3.5-1.56 3.5-3.5S12.44 5 10.5 5 7 6.56 7 8.5 8.56 12 10.5 12ZM13 8.5c0 1.38-1.13 2.5-2.5 2.5S8 9.87 8 8.5 9.13 6 10.5 6 13 7.13 13 8.5ZM6 5V4H2v1Z"
        /> < title > { title } < / title > < / svg >
    }
}
