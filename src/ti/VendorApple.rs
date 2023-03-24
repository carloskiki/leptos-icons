#[cfg(feature = "TiVendorApple")]
use leptos::*;
#[cfg(feature = "TiVendorApple")]
///This icon requires the feature `TiVendorApple` to be enabled.
#[component]
pub fn VendorApple(
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
        "M11.9 6.6s-.1-1.6.9-3l2.8-1.6s.1 1.6-.9 3l-2.8 1.6zM17.3 12.2c0-1.5.8-2.8 2-3.6l-.9-.9c-.5-.3-1.1-.7-2.4-.7-1.4 0-2.4.9-3.7.9-1.3 0-2.2-.8-3.1-.9-.7 0-1.4 0-2.1.3-.5.2-1.2.7-1.6 1.2-.6.6-1.2 1.9-1.3 3.1-.1 1.2-.1 2.1.2 3.2.2.9.6 1.8 1 2.6.3.6.6 1.2 1 1.8.3.4.7.8 1.1 1.1.3.2.6.4 1 .6.2 0 .5.1.8.1.6-.1 1.6-.9 2.4-1.1.4-.1.8-.1 1.3 0 .7.1 1.4.9 2.2 1 .6.1 1.2 0 1.7-.3.4-.2.7-.5 1-.9.4-.4.7-.9 1-1.3.4-.7.9-1.5 1.1-2.3-1.6-.6-2.7-2.1-2.7-3.9z"
        /> < title > { title } < / title > < / svg >
    }
}
