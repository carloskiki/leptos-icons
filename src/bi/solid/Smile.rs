#[cfg(feature = "BiSolidSmile")]
use leptos::*;
#[cfg(feature = "BiSolidSmile")]
///This icon requires the feature `BiSolidSmile` to be enabled.
#[component]
pub fn Smile(
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
        "M12 22c5.514 0 10-4.486 10-10S17.514 2 12 2 2 6.486 2 12s4.486 10 10 10zm3.493-13a1.494 1.494 0 1 1-.001 2.987A1.494 1.494 0 0 1 15.493 9zm-4.301 6.919a4.108 4.108 0 0 0 1.616 0c.253-.052.505-.131.75-.233.234-.1.464-.224.679-.368.208-.142.407-.306.591-.489.183-.182.347-.381.489-.592l1.658 1.117a6.027 6.027 0 0 1-1.619 1.621 6.003 6.003 0 0 1-2.149.904 6.116 6.116 0 0 1-2.414-.001 5.919 5.919 0 0 1-2.148-.903 6.078 6.078 0 0 1-1.621-1.622l1.658-1.117c.143.211.307.41.488.59a3.988 3.988 0 0 0 2.022 1.093zM8.5 9a1.5 1.5 0 1 1-.001 3.001A1.5 1.5 0 0 1 8.5 9z"
        /> < title > { title } < / title > < / svg >
    }
}
