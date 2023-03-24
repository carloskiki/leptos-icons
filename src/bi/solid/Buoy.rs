#[cfg(feature = "BiSolidBuoy")]
use leptos::*;
#[cfg(feature = "BiSolidBuoy")]
///This icon requires the feature `BiSolidBuoy` to be enabled.
#[component]
pub fn Buoy(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm7.736 8h-3.16A5.02 5.02 0 0 0 14 7.424V4.263A8.015 8.015 0 0 1 19.736 10zM12 15c-1.654 0-3-1.346-3-3s1.346-3 3-3 3 1.346 3 3-1.346 3-3 3zM10 4.263v3.161A5.02 5.02 0 0 0 7.424 10h-3.16A8.015 8.015 0 0 1 10 4.263zM4.264 14h3.16A5.02 5.02 0 0 0 10 16.576v3.161A8.015 8.015 0 0 1 4.264 14zM14 19.737v-3.161A5.02 5.02 0 0 0 16.576 14h3.16A8.015 8.015 0 0 1 14 19.737z"
        /> < title > { title } < / title > < / svg >
    }
}
