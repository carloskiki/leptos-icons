#[cfg(feature = "TiCreditCard")]
use leptos::*;
#[cfg(feature = "TiCreditCard")]
///This icon requires the feature `TiCreditCard` to be enabled.
#[component]
pub fn CreditCard(
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
        = "http://www.w3.org/2000/svg" > < g xmlns = "http://www.w3.org/2000/svg" >< path
        d =
        "M17 7h-11c-1.654 0-3 1.346-3 3v7c0 1.654 1.346 3 3 3h11c1.654 0 3-1.346 3-3v-7c0-1.654-1.346-3-3-3zm1 10c0 .552-.448 1-1 1h-11c-.552 0-1-.448-1-1v-4h13v4zm0-6h-13v-1c0-.552.448-1 1-1h11c.552 0 1 .448 1 1v1zM14 16h2c.276 0 .5-.224.5-.5s-.224-.5-.5-.5h-2c-.276 0-.5.224-.5.5s.224.5.5.5z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
