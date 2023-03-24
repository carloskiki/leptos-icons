#[cfg(feature = "AiOutlinedFork")]
use leptos::*;
#[cfg(feature = "AiOutlinedFork")]
///This icon requires the feature `AiOutlinedFork` to be enabled.
#[component]
pub fn Fork(
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
        stroke_witdh = "0" style = style class = "icon" viewBox = "0 0 1024 1024" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M752 100c-61.8 0-112 50.2-112 112 0 47.7 29.9 88.5 72 104.6v27.6L512 601.4 312 344.2v-27.6c42.1-16.1 72-56.9 72-104.6 0-61.8-50.2-112-112-112s-112 50.2-112 112c0 50.6 33.8 93.5 80 107.3v34.4c0 9.7 3.3 19.3 9.3 27L476 672.3v33.6c-44.2 15-76 56.9-76 106.1 0 61.8 50.2 112 112 112s112-50.2 112-112c0-49.2-31.8-91-76-106.1v-33.6l226.7-291.6c6-7.7 9.3-17.3 9.3-27v-34.4c46.2-13.8 80-56.7 80-107.3 0-61.8-50.2-112-112-112zM224 212a48.01 48.01 0 0 1 96 0 48.01 48.01 0 0 1-96 0zm336 600a48.01 48.01 0 0 1-96 0 48.01 48.01 0 0 1 96 0zm192-552a48.01 48.01 0 0 1 0-96 48.01 48.01 0 0 1 0 96z"
        /> < title > { title } < / title > < / svg >
    }
}
