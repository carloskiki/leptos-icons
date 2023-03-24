#[cfg(feature = "OcSmBellFill")]
use leptos::*;
#[cfg(feature = "OcSmBellFill")]
///This icon requires the feature `OcSmBellFill` to be enabled.
#[component]
pub fn BellFill(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M8 16c.9 0 1.7-.6 1.9-1.5.1-.3-.1-.5-.4-.5h-3c-.3 0-.5.2-.4.5.2.9 1 1.5 1.9 1.5ZM3 5c0-2.8 2.2-5 5-5s5 2.2 5 5v3l1.7 2.6c.2.2.3.5.3.8 0 .8-.7 1.5-1.5 1.5h-11c-.8.1-1.5-.6-1.5-1.4 0-.3.1-.6.3-.8L3 8.1V5Z"
        /> < title > { title } < / title > < / svg >
    }
}
