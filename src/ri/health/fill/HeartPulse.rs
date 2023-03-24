#[cfg(feature = "RiHealthFillHeartPulse")]
use leptos::*;
#[cfg(feature = "RiHealthFillHeartPulse")]
///This icon requires the feature `RiHealthFillHeartPulse` to be enabled.
#[component]
pub fn HeartPulse(
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
        "M16.5 3C19.538 3 22 5.5 22 9c0 7-7.5 11-10 12.5-1.978-1.187-7.084-3.937-9.132-8.5h4.698l.934-1.556 3 5L13.566 13H17v-2h-4.566l-.934 1.556-3-5L6.434 11H2.21C2.074 10.363 2 9.696 2 9c0-3.5 2.5-6 5.5-6C9.36 3 11 4 12 5c1-1 2.64-2 4.5-2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
