#[cfg(feature = "RiDeviceFillGps")]
use leptos::*;
#[cfg(feature = "RiDeviceFillGps")]
///This icon requires the feature `RiDeviceFillGps` to be enabled.
#[component]
pub fn Gps(
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
        "M12 16l3 6H9l3-6zm-2.627.255a5 5 0 1 1 5.255 0l-1.356-2.711a2 2 0 1 0-2.544 0l-1.355 2.71zm-2.241 4.482A9.997 9.997 0 0 1 2 12C2 6.477 6.477 2 12 2s10 4.477 10 10a9.997 9.997 0 0 1-5.132 8.737l-1.343-2.688a7 7 0 1 0-7.05 0l-1.343 2.688z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
