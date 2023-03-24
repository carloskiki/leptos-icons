#[cfg(feature = "FaSolidCarOn")]
use leptos::*;
#[cfg(feature = "FaSolidCarOn")]
///This icon requires the feature `FaSolidCarOn` to be enabled.
#[component]
pub fn CarOn(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M248 24c0-13.3-10.7-24-24-24s-24 10.7-24 24v80c0 13.3 10.7 24 24 24s24-10.7 24-24V24zM153.8 224H294.2c6.8 0 12.8 4.3 15.1 10.6L328.3 288H119.7l19.1-53.4c2.3-6.4 8.3-10.6 15.1-10.6zM78.5 213.1L50.2 292.4C30.1 300.9 16 320.8 16 344v40 64 32c0 17.7 14.3 32 32 32H64c17.7 0 32-14.3 32-32V448H352v32c0 17.7 14.3 32 32 32h16c17.7 0 32-14.3 32-32V448 384 344c0-23.2-14.1-43.1-34.2-51.6l-28.3-79.3C358.1 181.3 328 160 294.2 160H153.8c-33.8 0-64 21.3-75.3 53.1zM96 344a24 24 0 1 1 0 48 24 24 0 1 1 0-48zm232 24a24 24 0 1 1 48 0 24 24 0 1 1 -48 0zM7 39C-2.3 48.4-2.3 63.6 7 73l48 48c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9L41 39C31.6 29.7 16.4 29.7 7 39zm400 0L359 87c-9.4 9.4-9.4 24.6 0 33.9s24.6 9.4 33.9 0l48-48c9.4-9.4 9.4-24.6 0-33.9s-24.6-9.4-33.9 0z"
        /> < title > { title } < / title > < / svg >
    }
}
