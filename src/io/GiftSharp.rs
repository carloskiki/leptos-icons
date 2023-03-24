#[cfg(feature = "IoGiftSharp")]
use leptos::*;
#[cfg(feature = "IoGiftSharp")]
///This icon requires the feature `IoGiftSharp` to be enabled.
#[component]
pub fn GiftSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M346,110a34,34,0,0,0-68,0v34h34A34,34,0,0,0,346,110Z" style = "fill:none" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M234,110a34,34,0,1,0-34,34h34Z"
        style = "fill:none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M234,144h44V256H442a22,22,0,0,0,22-22V166a22,22,0,0,0-22-22H382.18A77.95,77.95,0,0,0,256,55.79,78,78,0,0,0,129.81,144H70a22,22,0,0,0-22,22v68a22,22,0,0,0,22,22H234Zm44-34a34,34,0,1,1,34,34H278Zm-112,0a34,34,0,1,1,68,0v34H200A34,34,0,0,1,166,110Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M278,480H410a22,22,0,0,0,22-22V288H278Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M80,458a22,22,0,0,0,22,22H234V288H80Z" /> <
        title > { title } < / title > < / svg >
    }
}
