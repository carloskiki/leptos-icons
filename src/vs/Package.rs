#[cfg(feature = "VsPackage")]
use leptos::*;
#[cfg(feature = "VsPackage")]
///This icon requires the feature `VsPackage` to be enabled.
#[component]
pub fn Package(
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
        "M8.61 3l5.74 1.53L15 5v6.74l-.37.48-6.13 1.69-6.14-1.69-.36-.48V5l.61-.47L8.34 3h.27zm-.09 1l-4 1 .55.2 3.43.9 3-.81.95-.29-3.93-1zM3 11.36l5 1.37V7L3 5.66v5.7zM9 7v5.73l5-1.37V5.63l-2.02.553V8.75l-1 .26V6.457L9 7z"
        /> < title > { title } < / title > < / svg >
    }
}
