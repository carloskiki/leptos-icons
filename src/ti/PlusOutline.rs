#[cfg(feature = "TiPlusOutline")]
use leptos::*;
#[cfg(feature = "TiPlusOutline")]
///This icon requires the feature `TiPlusOutline` to be enabled.
#[component]
pub fn PlusOutline(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 21c-1.654 0-3-1.346-3-3l.053-3.053-3.035.053c-1.672 0-3.018-1.346-3.018-3s1.346-3 3-3l3.053-.054-.053-2.928c0-1.672 1.346-3.018 3-3.018s3 1.346 3 3l.055 2.946 2.963.054c1.636 0 2.982 1.346 2.982 3s-1.346 3-3 3l-2.945-.053-.055 3.071c0 1.636-1.346 2.982-3 2.982zm-1-8v5.018c0 .533.449.982 1 .982s1-.449 1-1v-5h5.018c.533 0 .982-.449.982-1s-.449-1-1-1h-5v-5c0-.569-.449-1-1-1s-1 .449-1 1v5h-5c-.569 0-1 .449-1 1s.449 1 1 1h5z"
        /> < title > { title } < / title > < / svg >
    }
}
