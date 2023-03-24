#[cfg(feature = "RiDesignFillLayout4")]
use leptos::*;
#[cfg(feature = "RiDesignFillLayout4")]
///This icon requires the feature `RiDesignFillLayout4` to be enabled.
#[component]
pub fn Layout4(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M11 13v8H4a1 1 0 0 1-1-1v-7h8zm2-10h7a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1h-7V3zM3 4a1 1 0 0 1 1-1h7v8H3V4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
