#[cfg(feature = "SiDunked")]
use leptos::*;
#[cfg(feature = "SiDunked")]
///This icon requires the feature `SiDunked` to be enabled.
#[component]
pub fn Dunked(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M13.799 0v19.8599A4.2002 4.2002 0 0018.0003 24h4.2002V4.1411A4.2002 4.2002 0 0017.9992 0H13.798zM6.2983 15.0014a4.5008 4.5008 0 00-4.4988 4.3906v.2224a4.5008 4.5008 0 008.9986 0v-.2154a4.5008 4.5008 0 00-4.4998-4.3986z"
        /> < title > { title } < / title > < / svg >
    }
}
