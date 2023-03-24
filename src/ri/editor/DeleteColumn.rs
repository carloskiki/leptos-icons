#[cfg(feature = "RiEditorDeleteColumn")]
use leptos::*;
#[cfg(feature = "RiEditorDeleteColumn")]
///This icon requires the feature `RiEditorDeleteColumn` to be enabled.
#[component]
pub fn DeleteColumn(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M12 3c.552 0 1 .448 1 1v8c.835-.628 1.874-1 3-1 2.761 0 5 2.239 5 5s-2.239 5-5 5c-1.032 0-1.99-.313-2.787-.848L13 20c0 .552-.448 1-1 1H6c-.552 0-1-.448-1-1V4c0-.552.448-1 1-1h6zm-1 2H7v14h4V5zm8 10h-6v2h6v-2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
