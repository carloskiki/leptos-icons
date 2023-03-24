#[cfg(feature = "VsLayoutMenubar")]
use leptos::*;
#[cfg(feature = "VsLayoutMenubar")]
///This icon requires the feature `VsLayoutMenubar` to be enabled.
#[component]
pub fn LayoutMenubar(
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
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M1 2.00085L2 1.00085H14L15 2.00085V14.0009L14 15.0009H2L1 14.0009V2.00085ZM2 2.00085V14.0009H14V2.00085H2ZM3 3.00085H5V4.00085H3V3.00085ZM6 3.00085H8V4.00085H6V3.00085ZM11 3.00085H9V4.00085H11V3.00085Z"
        /> < title > { title } < / title > < / svg >
    }
}
