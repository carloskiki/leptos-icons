#[cfg(feature = "SiLinuxfoundation")]
use leptos::*;
#[cfg(feature = "SiLinuxfoundation")]
///This icon requires the feature `SiLinuxfoundation` to be enabled.
#[component]
pub fn Linuxfoundation(
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
        = "M4.8 19.2h9.6V24H0V9.6h4.8v9.6zM0 0v7.2h4.8V4.822h14.4V19.2h-2.4V24H24V0H0z"
        /> < title > { title } < / title > < / svg >
    }
}
