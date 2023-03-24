#[cfg(feature = "BiLogosVenmo")]
use leptos::*;
#[cfg(feature = "BiLogosVenmo")]
///This icon requires the feature `BiLogosVenmo` to be enabled.
#[component]
pub fn Venmo(
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
        "M14 3.27a7.49 7.49 0 0 1 .66 3.35c0 2.72-1.93 6.72-3.49 9.27L9.53 2.44l-6.91.65L5.79 22h7.88c3.45-4.54 7.71-11 7.71-16a7.3 7.3 0 0 0-1.06-4z"
        /> < title > { title } < / title > < / svg >
    }
}
