#[cfg(feature = "TiVendorMicrosoft")]
use leptos::*;
#[cfg(feature = "TiVendorMicrosoft")]
///This icon requires the feature `TiVendorMicrosoft` to be enabled.
#[component]
pub fn VendorMicrosoft(
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
        "M10 12.5c0-.3-.2-.5-.5-.5h-6c-.3 0-.5.2-.5.5v5c0 .3.2.5.5.6l6 .7c.3 0 .5-.2.5-.4v-5.9zM11.5 12c-.3 0-.5.2-.5.5v5.9c0 .3.2.5.5.6l9 1c.3 0 .5-.2.5-.4v-7c0-.3-.2-.5-.5-.5l-9-.1zM10 4.7c0-.3-.2-.5-.5-.4l-6 .7c-.3 0-.5.2-.5.5v5c0 .3.2.5.5.5h6c.3 0 .5-.2.5-.5v-5.8zM11.5 4.1c-.3 0-.5.3-.5.6v5.9c0 .3.2.5.5.5h9c.3 0 .5-.2.5-.5v-7c0-.3-.2-.5-.5-.4l-9 .9z"
        /> < title > { title } < / title > < / svg >
    }
}
