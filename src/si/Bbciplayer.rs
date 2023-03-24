#[cfg(feature = "SiBbciplayer")]
use leptos::*;
#[cfg(feature = "SiBbciplayer")]
///This icon requires the feature `SiBbciplayer` to be enabled.
#[component]
pub fn Bbciplayer(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M8.315 0H2.382v6.022h5.933V3.506l9.618 8.45-9.618 8.538V8.99H2.382V24h5.933l13.303-12.045Z"
        /> < title > { title } < / title > < / svg >
    }
}
