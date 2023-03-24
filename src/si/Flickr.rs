#[cfg(feature = "SiFlickr")]
use leptos::*;
#[cfg(feature = "SiFlickr")]
///This icon requires the feature `SiFlickr` to be enabled.
#[component]
pub fn Flickr(
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
        "M5.334 6.666C2.3884 6.666 0 9.055 0 12c0 2.9456 2.3884 5.334 5.334 5.334 2.9456 0 5.332-2.3884 5.332-5.334 0-2.945-2.3864-5.334-5.332-5.334zm13.332 0c-2.9456 0-5.332 2.389-5.332 5.334 0 2.9456 2.3864 5.334 5.332 5.334C21.6116 17.334 24 14.9456 24 12c0-2.945-2.3884-5.334-5.334-5.334Z"
        /> < title > { title } < / title > < / svg >
    }
}
