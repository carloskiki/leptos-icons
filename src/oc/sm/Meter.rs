#[cfg(feature = "OcSmMeter")]
use leptos::*;
#[cfg(feature = "OcSmMeter")]
///This icon requires the feature `OcSmMeter` to be enabled.
#[component]
pub fn Meter(
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
        "M8 1.5a6.5 6.5 0 1 0 6.016 4.035.75.75 0 0 1 1.388-.57 8 8 0 1 1-4.37-4.37.75.75 0 1 1-.569 1.389A6.473 6.473 0 0 0 8 1.5Zm6.28.22a.75.75 0 0 1 0 1.06l-4.063 4.064a2.5 2.5 0 1 1-1.06-1.06L13.22 1.72a.75.75 0 0 1 1.06 0ZM7 8a1 1 0 1 0 2 0 1 1 0 0 0-2 0Z"
        /> < title > { title } < / title > < / svg >
    }
}
