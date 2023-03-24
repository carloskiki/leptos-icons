#[cfg(feature = "RiOthersLineFridge")]
use leptos::*;
#[cfg(feature = "RiOthersLineFridge")]
///This icon requires the feature `RiOthersLineFridge` to be enabled.
#[component]
pub fn Fridge(
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
        "M19 1c.552 0 1 .448 1 1v20c0 .552-.448 1-1 1H5c-.552 0-1-.448-1-1V2c0-.552.448-1 1-1h14zm-1 11H6v9h12v-9zm-8 2v4H8v-4h2zm8-11H6v7h12V3zm-8 2v3H8V5h2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
