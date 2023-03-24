#[cfg(feature = "HiLgSolidBolt")]
use leptos::*;
#[cfg(feature = "HiLgSolidBolt")]
///This icon requires the feature `HiLgSolidBolt` to be enabled.
#[component]
pub fn Bolt(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M14.6152 1.59495C14.9164 1.76289 15.0643 2.11463 14.9736 2.44736L12.9819 9.75003H20.25C20.5486 9.75003 20.8188 9.92721 20.9378 10.2011C21.0569 10.475 21.0021 10.7934 20.7983 11.0118L10.2983 22.2618C10.063 22.5139 9.68601 22.573 9.38478 22.4051C9.08354 22.2372 8.93567 21.8854 9.02641 21.5527L11.018 14.25H3.74999C3.45134 14.25 3.18115 14.0728 3.06213 13.7989C2.9431 13.525 2.99792 13.2066 3.20169 12.9883L13.7017 1.73829C13.937 1.48615 14.314 1.42701 14.6152 1.59495Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
