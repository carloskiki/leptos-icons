#[cfg(feature = "SiShadow")]
use leptos::*;
#[cfg(feature = "SiShadow")]
///This icon requires the feature `SiShadow` to be enabled.
#[component]
pub fn Shadow(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 0C5.3727 0 0 5.3727 0 12c0 3.5145 1.511 6.6754 3.9181 8.8702a4.457 4.457 0 01-.1998-1.3238c0-2.4597 1.9938-4.4535 4.4536-4.4535 2.4596 0 4.4535 1.9938 4.4535 4.4535 0 1.9565-1.262 3.6171-3.016 4.2153C10.382 23.9178 11.1815 24 12 24c6.6273 0 12-5.3727 12-12S18.6273 0 12 0Z"
        /> < title > { title } < / title > < / svg >
    }
}
