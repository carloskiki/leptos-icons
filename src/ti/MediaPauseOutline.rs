#[cfg(feature = "TiMediaPauseOutline")]
use leptos::*;
#[cfg(feature = "TiMediaPauseOutline")]
///This icon requires the feature `TiMediaPauseOutline` to be enabled.
#[component]
pub fn MediaPauseOutline(
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
        "M8 20c-1.654 0-3-1.346-3-3v-9c0-1.654 1.346-3 3-3s3 1.346 3 3v9c0 1.654-1.346 3-3 3zm0-13c-.552 0-1 .449-1 1v9c0 .551.448 1 1 1s1-.449 1-1v-9c0-.551-.448-1-1-1zM15 20c-1.654 0-3-1.346-3-3v-9c0-1.654 1.346-3 3-3s3 1.346 3 3v9c0 1.654-1.346 3-3 3zm0-13c-.552 0-1 .449-1 1v9c0 .551.448 1 1 1s1-.449 1-1v-9c0-.551-.448-1-1-1z"
        /> < title > { title } < / title > < / svg >
    }
}
