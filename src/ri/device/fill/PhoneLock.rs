#[cfg(feature = "RiDeviceFillPhoneLock")]
use leptos::*;
#[cfg(feature = "RiDeviceFillPhoneLock")]
///This icon requires the feature `RiDeviceFillPhoneLock` to be enabled.
#[component]
pub fn PhoneLock(
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
        "M18 2a1 1 0 0 1 1 1l.001 7.1A5.002 5.002 0 0 0 13.1 14H12v8H6a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1h12zm0 10a3 3 0 0 1 3 3v1h1v5a1 1 0 0 1-1 1h-6a1 1 0 0 1-1-1v-5h1v-1a3 3 0 0 1 3-3zm0 2c-.513 0-1 .45-1 1v1h2v-1a1 1 0 0 0-1-1z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
