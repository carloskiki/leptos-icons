#[cfg(feature = "FaSolidPlaceOfWorship")]
use leptos::*;
#[cfg(feature = "FaSolidPlaceOfWorship")]
///This icon requires the feature `FaSolidPlaceOfWorship` to be enabled.
#[component]
pub fn PlaceOfWorship(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M224 109.3V217.6L183.3 242c-14.5 8.7-23.3 24.3-23.3 41.2V512h96V416c0-35.3 28.7-64 64-64s64 28.7 64 64v96h96V283.2c0-16.9-8.8-32.5-23.3-41.2L416 217.6V109.3c0-8.5-3.4-16.6-9.4-22.6L331.3 11.3c-6.2-6.2-16.4-6.2-22.6 0L233.4 86.6c-6 6-9.4 14.1-9.4 22.6zM24.9 330.3C9.5 338.8 0 354.9 0 372.4V464c0 26.5 21.5 48 48 48h80V273.6L24.9 330.3zM592 512c26.5 0 48-21.5 48-48V372.4c0-17.5-9.5-33.6-24.9-42.1L512 273.6V512h80z"
        /> < title > { title } < / title > < / svg >
    }
}
