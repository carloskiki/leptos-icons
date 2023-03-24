#[cfg(feature = "ImLastfm")]
use leptos::*;
#[cfg(feature = "ImLastfm")]
///This icon requires the feature `ImLastfm` to be enabled.
#[component]
pub fn Lastfm(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M7.056 11.972l-0.588-1.594c0 0-0.953 1.063-2.381 1.063-1.266 0-2.163-1.1-2.163-2.859 0-2.253 1.137-3.059 2.253-3.059 1.612 0 2.125 1.044 2.566 2.381l0.588 1.831c0.588 1.778 1.688 3.206 4.856 3.206 2.272 0 3.813-0.697 3.813-2.528 0-1.484-0.844-2.253-2.419-2.622l-1.172-0.256c-0.806-0.184-1.044-0.513-1.044-1.063 0-0.622 0.494-0.991 1.3-0.991 0.881 0 1.356 0.331 1.428 1.119l1.831-0.219c-0.147-1.65-1.284-2.328-3.153-2.328-1.65 0-3.262 0.622-3.262 2.622 0 1.247 0.606 2.034 2.125 2.4l1.247 0.294c0.934 0.219 1.247 0.606 1.247 1.137 0 0.678-0.659 0.953-1.906 0.953-1.85 0-2.622-0.972-3.059-2.309l-0.606-1.831c-0.766-2.384-1.994-3.263-4.431-3.263-2.694 0-4.125 1.703-4.125 4.6 0 2.784 1.428 4.287 3.997 4.287 2.069 0 3.059-0.972 3.059-0.972v0z"
        /> < title > { title } < / title > < / svg >
    }
}
