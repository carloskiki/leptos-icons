#[cfg(feature = "SiYoutubegaming")]
use leptos::*;
#[cfg(feature = "SiYoutubegaming")]
///This icon requires the feature `SiYoutubegaming` to be enabled.
#[component]
pub fn Youtubegaming(
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
        "M24 13.2v-6l-6-3.6-6 3.6-6-3.6-6 3.6v6l12 7.2zM8.4 10.8H6v2.4H4.8v-2.4H2.4V9.6h2.4V7.2H6v2.4h2.4zm7.2 2.4a1.2 1.2 0 01-1.2-1.2c0-.66.54-1.2 1.2-1.2.66 0 1.2.54 1.2 1.2 0 .66-.54 1.2-1.2 1.2zm3.6-2.4A1.2 1.2 0 0118 9.6c0-.66.54-1.2 1.2-1.2.66 0 1.2.54 1.2 1.2 0 .66-.54 1.2-1.2 1.2Z"
        /> < title > { title } < / title > < / svg >
    }
}
