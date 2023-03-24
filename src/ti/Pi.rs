#[cfg(feature = "TiPi")]
use leptos::*;
#[cfg(feature = "TiPi")]
///This icon requires the feature `TiPi` to be enabled.
#[component]
pub fn Pi(
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
        "M18.707 8.535c-.391-.391-1.023-.391-1.414 0-1.264 1.264-3.321 1.264-4.586 0-2.045-2.044-5.371-2.042-7.414 0-.391.391-.391 1.023 0 1.414s1.023.391 1.414 0c.374-.374.82-.624 1.293-.776v7.827c0 .553.447 1 1 1s1-.447 1-1v-7.826c.472.152.919.401 1.293.775.768.767 1.715 1.245 2.707 1.437v5.614c0 .553.447 1 1 1s1-.447 1-1v-5.614c.992-.191 1.939-.67 2.707-1.437.391-.39.391-1.023 0-1.414z"
        /> < title > { title } < / title > < / svg >
    }
}
