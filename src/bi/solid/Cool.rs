#[cfg(feature = "BiSolidCool")]
use leptos::*;
#[cfg(feature = "BiSolidCool")]
///This icon requires the feature `BiSolidCool` to be enabled.
#[component]
pub fn Cool(
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
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm4.095 14.244a5.982 5.982 0 0 1-3.034 1.634 6.05 6.05 0 0 1-2.414 0 5.919 5.919 0 0 1-2.148-.903 6.078 6.078 0 0 1-1.621-1.622l1.658-1.117c.143.211.307.41.488.59a3.988 3.988 0 0 0 1.273.86c.243.102.495.181.749.232a4.108 4.108 0 0 0 1.616 0c.253-.052.505-.131.75-.233.234-.1.464-.224.679-.368.208-.142.407-.306.591-.489.183-.182.347-.381.489-.592l1.658 1.117c-.215.32-.462.62-.734.891zM19 10a2 2 0 0 1-2 2h-2a2 2 0 0 1-2-2h-2a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V8h14v2z"
        /> < title > { title } < / title > < / svg >
    }
}
