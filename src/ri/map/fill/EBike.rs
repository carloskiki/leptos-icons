#[cfg(feature = "RiMapFillEBike")]
use leptos::*;
#[cfg(feature = "RiMapFillEBike")]
///This icon requires the feature `RiMapFillEBike` to be enabled.
#[component]
pub fn EBike(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path d =
        "M15.5 6.937A6.997 6.997 0 0 1 19 13v8h-4.17a3.001 3.001 0 0 1-5.66 0H5v-8a6.997 6.997 0 0 1 3.5-6.063A3.974 3.974 0 0 1 8.125 6H5V4h3.126a4.002 4.002 0 0 1 7.748 0H19v2h-3.126c-.085.33-.212.645-.373.937zM12 14a1 1 0 0 0-1 1v5a1 1 0 0 0 2 0v-5a1 1 0 0 0-1-1zm0-7a2 2 0 1 0 0-4 2 2 0 0 0 0 4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
