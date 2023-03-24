#[cfg(feature = "SiExpressvpn")]
use leptos::*;
#[cfg(feature = "SiExpressvpn")]
///This icon requires the feature `SiExpressvpn` to be enabled.
#[component]
pub fn Expressvpn(
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
        "M11.705 2.349a4.874 4.874 0 00-4.39 2.797L6.033 7.893h14.606c.41 0 .692.308.692.668 0 .359-.282.666-.692.666H2.592L0 14.772h2.824c-.796 1.72-1.002 2.567-1.002 3.26 0 2.105 1.72 3.62 4.416 3.62h8.239c1.771 0 3.337-1.412 3.337-3.03 0-1.411-1.206-2.515-2.772-2.515H5.596c-.873 0-1.284-.59-.924-1.335h11.859c4.004 0 7.469-3.029 7.469-6.802 0-3.183-2.618-5.621-6.16-5.621z"
        /> < title > { title } < / title > < / svg >
    }
}
