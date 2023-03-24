#[cfg(feature = "SiPleroma")]
use leptos::*;
#[cfg(feature = "SiPleroma")]
///This icon requires the feature `SiPleroma` to be enabled.
#[component]
pub fn Pleroma(
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
        "M6.36 0A1.868 1.868 0 004.49 1.868V24h5.964V0zm7.113 0v12h4.168a1.868 1.868 0 001.868-1.868V0zm0 18.036V24h4.168a1.868 1.868 0 001.868-1.868v-4.096Z"
        /> < title > { title } < / title > < / svg >
    }
}
