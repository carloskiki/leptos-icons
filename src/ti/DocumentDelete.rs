#[cfg(feature = "TiDocumentDelete")]
use leptos::*;
#[cfg(feature = "TiDocumentDelete")]
///This icon requires the feature `TiDocumentDelete` to be enabled.
#[component]
pub fn DocumentDelete(
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
        "M19.707 7.293l-4-4c-.187-.188-.441-.293-.707-.293h-8c-1.654 0-3 1.346-3 3v12c0 1.654 1.346 3 3 3h10c1.654 0 3-1.346 3-3v-10c0-.266-.105-.52-.293-.707zm-2.121.707h-1.086c-.827 0-1.5-.673-1.5-1.5v-1.086l2.586 2.586zm-.586 11h-10c-.552 0-1-.448-1-1v-12c0-.552.448-1 1-1h7v1.5c0 1.379 1.121 2.5 2.5 2.5h1.5v9c0 .552-.448 1-1 1zM15 14h-6c-.553 0-1-.447-1-1s.447-1 1-1h6c.553 0 1 .447 1 1s-.447 1-1 1z"
        /> < title > { title } < / title > < / svg >
    }
}
