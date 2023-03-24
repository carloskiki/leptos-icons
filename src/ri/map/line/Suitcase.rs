#[cfg(feature = "RiMapLineSuitcase")]
use leptos::*;
#[cfg(feature = "RiMapLineSuitcase")]
///This icon requires the feature `RiMapLineSuitcase` to be enabled.
#[component]
pub fn Suitcase(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0H24V24H0z" />< path d =
        "M15 3c.552 0 1 .448 1 1v2h5c.552 0 1 .448 1 1v13c0 .552-.448 1-1 1H3c-.552 0-1-.448-1-1V7c0-.552.448-1 1-1h5V4c0-.552.448-1 1-1h6zm1 5H8v11h8V8zM4 8v11h2V8H4zm10-3h-4v1h4V5zm4 3v11h2V8h-2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
