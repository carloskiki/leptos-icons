#[cfg(feature = "FaSolidTrailer")]
use leptos::*;
#[cfg(feature = "FaSolidTrailer")]
///This icon requires the feature `FaSolidTrailer` to be enabled.
#[component]
pub fn Trailer(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M48 32C21.5 32 0 53.5 0 80V336c0 26.5 21.5 48 48 48H65.1c7.8-54.3 54.4-96 110.9-96s103.1 41.7 110.9 96H488h8H608c17.7 0 32-14.3 32-32s-14.3-32-32-32H544V80c0-26.5-21.5-48-48-48H48zM80 96c8.8 0 16 7.2 16 16l0 131.2c-11.4 5.9-22.2 12.9-32 21V112c0-8.8 7.2-16 16-16zm96 128c-5.4 0-10.7 .2-16 .7L160 112c0-8.8 7.2-16 16-16s16 7.2 16 16l0 112.7c-5.3-.5-10.6-.7-16-.7zm80 19.2L256 112c0-8.8 7.2-16 16-16s16 7.2 16 16l0 152.2c-9.8-8.1-20.6-15.2-32-21zM368 96c8.8 0 16 7.2 16 16l0 192c0 8.8-7.2 16-16 16s-16-7.2-16-16l0-192c0-8.8 7.2-16 16-16zm112 16l0 192c0 8.8-7.2 16-16 16s-16-7.2-16-16l0-192c0-8.8 7.2-16 16-16s16 7.2 16 16zM176 480a80 80 0 1 0 0-160 80 80 0 1 0 0 160zm0-112a32 32 0 1 1 0 64 32 32 0 1 1 0-64z"
        /> < title > { title } < / title > < / svg >
    }
}
