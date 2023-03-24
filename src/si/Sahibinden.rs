#[cfg(feature = "SiSahibinden")]
use leptos::*;
#[cfg(feature = "SiSahibinden")]
///This icon requires the feature `SiSahibinden` to be enabled.
#[component]
pub fn Sahibinden(
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
        "M0 0v24h24V0zm11.517 4.723c.563-.007 1.13-.004 1.69.063 2.412.054 4.853 2.18 4.879 4.508h-3.319c.009-.694-.603-1.555-1.279-1.732-1.105-.269-2.46-.355-3.43.294-.738.445-1.065 1.672-.095 2.056 2.288 1.083 5.158.846 7.224 2.372 1.698 1.21 1.598 3.666.274 5.086-1.718 1.84-4.636 2.132-7.099 1.782-2.448-.117-4.755-2.245-4.819-4.562h3.311c-.056.832.638 1.557 1.46 1.822 1.27.275 2.726.358 3.93-.19.96-.323 1.024-1.544.284-2.103-1.595-.897-3.565-.924-5.297-1.518-2.012-.39-3.643-2.278-3.26-4.197.424-2.342 3.127-3.727 5.546-3.681z"
        /> < title > { title } < / title > < / svg >
    }
}
