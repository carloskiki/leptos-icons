#[cfg(feature = "BiSolidGhost")]
use leptos::*;
#[cfg(feature = "BiSolidGhost")]
///This icon requires the feature `BiSolidGhost` to be enabled.
#[component]
pub fn Ghost(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M3 11v8h.051c.245 1.692 1.69 3 3.449 3 1.174 0 2.074-.417 2.672-1.174a3.99 3.99 0 0 0 5.668-.014c.601.762 1.504 1.188 2.66 1.188 1.93 0 3.5-1.57 3.5-3.5V11c0-4.962-4.037-9-9-9s-9 4.038-9 9zm6 1c-1.103 0-2-.897-2-2s.897-2 2-2 2 .897 2 2-.897 2-2 2zm6-4c1.103 0 2 .897 2 2s-.897 2-2 2-2-.897-2-2 .897-2 2-2z"
        /> < title > { title } < / title > < / svg >
    }
}
