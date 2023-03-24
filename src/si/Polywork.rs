#[cfg(feature = "SiPolywork")]
use leptos::*;
#[cfg(feature = "SiPolywork")]
///This icon requires the feature `SiPolywork` to be enabled.
#[component]
pub fn Polywork(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M19.125 0H4.875A4.865 4.865 0 0 0 0 4.875v14.25C0 21.825 2.175 24 4.875 24h6.6c2.7 0 4.875-2.175 4.875-4.875V16.65h2.775c2.7 0 4.875-2.175 4.875-4.875v-6.9C24 2.175 21.825 0 19.125 0zM16.5 1.275h2.625a3.6 3.6 0 0 1 3.6 3.6v2.7H16.5v-6.3zM15.075 9v6.45H8.85V9h6.225zM8.85 1.2h6.225v6.375H8.85V1.2zM1.275 4.8a3.6 3.6 0 0 1 3.6-3.6H7.5v6.375H1.275V4.8zM7.5 9v6.45H1.2V9h6.3zm0 13.725H4.8a3.6 3.6 0 0 1-3.6-3.6V16.8h6.3v5.925zm7.575-3.525a3.6 3.6 0 0 1-3.6 3.6H8.85v-5.925h6.225V19.2zm7.65-7.35a3.6 3.6 0 0 1-3.6 3.6H16.5V9h6.225v2.85z"
        /> < title > { title } < / title > < / svg >
    }
}
