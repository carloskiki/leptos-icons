#[cfg(feature = "TiCamera")]
use leptos::*;
#[cfg(feature = "TiCamera")]
///This icon requires the feature `TiCamera` to be enabled.
#[component]
pub fn Camera(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = { size.clone() } height = { size
        } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 6h-1.586l-1-1c-.579-.579-1.595-1-2.414-1h-4c-.819 0-1.835.421-2.414 1l-1 1h-1.586c-1.654 0-3 1.346-3 3v8c0 1.654 1.346 3 3 3h14c1.654 0 3-1.346 3-3v-8c0-1.654-1.346-3-3-3zm-7 10c-1.933 0-3.5-1.568-3.5-3.5 0-1.934 1.567-3.5 3.5-3.5s3.5 1.566 3.5 3.5c0 1.932-1.567 3.5-3.5 3.5zm6-4.701c-.719 0-1.3-.58-1.3-1.299s.581-1.301 1.3-1.301 1.3.582 1.3 1.301-.581 1.299-1.3 1.299z"
        /> < title > { title } < / title > < / svg >
    }
}
