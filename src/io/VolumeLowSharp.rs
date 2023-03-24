#[cfg(feature = "IoVolumeLowSharp")]
use leptos::*;
#[cfg(feature = "IoVolumeLowSharp")]
///This icon requires the feature `IoVolumeLowSharp` to be enabled.
#[component]
pub fn VolumeLowSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M391.12,341.48l-28.6-14.36,7.18-14.3c9.49-18.9,14.3-38,14.3-56.82,0-19.36-4.66-37.92-14.25-56.73L362.48,185,391,170.48l7.26,14.25C410.2,208.16,416,231.47,416,256c0,23.83-6,47.78-17.7,71.18Z"
        />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "189.65 176.1 96 176.1 96 335.9 189.65 335.9 320 440 320 72 189.65 176.1" /> <
        title > { title } < / title > < / svg >
    }
}
