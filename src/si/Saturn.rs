#[cfg(feature = "SiSaturn")]
use leptos::*;
#[cfg(feature = "SiSaturn")]
///This icon requires the feature `SiSaturn` to be enabled.
#[component]
pub fn Saturn(
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
        "M19.039 11.459c.001.015.022.244.03.407.006.113 0 .29 0 .3.003 0 .029.023.03.024 1.428 1.17 2.943 2.767 3.204 3.94.073.325.056.618-.072.868-.152.293-.439.503-.834.638-2.046.7-6.925-.642-10.907-2.609-2.845-1.406-5.342-3.081-7.032-4.719-1.57-1.523-1.995-2.71-1.59-3.427.155-.271.42-.472.776-.609 1.299-.507 3.788-.152 6.239.579-1.16.866-1.968 2.034-2.342 3.202l-.001.007a.051.051 0 0 1-.001.006c-.115 1.07 1.434 2.47 3 3.25-.002-.006.084.032.084.026-.002-.006-.015-.109-.017-.113-.366-2.66 1.648-6.64 3.765-7.513.136-.056.254-.09.27-.095l-.273-.027c-.074-.006-.148-.013-.228-.015a7.464 7.464 0 0 0-.272-.01 6.443 6.443 0 0 0-3.4.892C5.378 5.057 2.383 4.892 1.13 5.31c-.497.167-.833.418-1 .751-.174.35-.175.79-.002 1.306.57 1.704 3.058 4.032 6.211 6.099.457 2.407 2.615 4.875 5.703 5.204l.142.015a.278.278 0 0 1 .05 0 6.618 6.618 0 0 0-.173-.132c-.955-.736-1.813-1.949-2.107-3l.185.093.143.07c4.985 2.465 10.215 3.72 12.53 2.947.519-.174.9-.418 1.075-.768.167-.335.139-.78-.029-1.278-.436-1.3-2.304-3.284-4.675-5.052a5.003 5.003 0 0 0-.145-.107"
        /> < title > { title } < / title > < / svg >
    }
}
