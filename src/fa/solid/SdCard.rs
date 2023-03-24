#[cfg(feature = "FaSolidSdCard")]
use leptos::*;
#[cfg(feature = "FaSolidSdCard")]
///This icon requires the feature `FaSolidSdCard` to be enabled.
#[component]
pub fn SdCard(
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
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M320 0H141.3C124.3 0 108 6.7 96 18.7L18.7 96C6.7 108 0 124.3 0 141.3V448c0 35.3 28.7 64 64 64H320c35.3 0 64-28.7 64-64V64c0-35.3-28.7-64-64-64zM160 88v48c0 13.3-10.7 24-24 24s-24-10.7-24-24V88c0-13.3 10.7-24 24-24s24 10.7 24 24zm80 0v48c0 13.3-10.7 24-24 24s-24-10.7-24-24V88c0-13.3 10.7-24 24-24s24 10.7 24 24zm80 0v48c0 13.3-10.7 24-24 24s-24-10.7-24-24V88c0-13.3 10.7-24 24-24s24 10.7 24 24z"
        /> < title > { title } < / title > < / svg >
    }
}
