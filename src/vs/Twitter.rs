#[cfg(feature = "VsTwitter")]
use leptos::*;
#[cfg(feature = "VsTwitter")]
///This icon requires the feature `VsTwitter` to be enabled.
#[component]
pub fn Twitter(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 3.784a5.63 5.63 0 0 1-.65.803 6.058 6.058 0 0 1-.786.68 5.442 5.442 0 0 1 .014.377c0 .574-.061 1.141-.184 1.702a8.467 8.467 0 0 1-.534 1.627 8.444 8.444 0 0 1-1.264 2.04 7.768 7.768 0 0 1-1.72 1.521 7.835 7.835 0 0 1-2.095.95 8.524 8.524 0 0 1-2.379.329 8.178 8.178 0 0 1-2.293-.325A7.921 7.921 0 0 1 1 12.52a5.762 5.762 0 0 0 4.252-1.19 2.842 2.842 0 0 1-2.273-1.19 2.878 2.878 0 0 1-.407-.8c.091.014.181.026.27.035a2.797 2.797 0 0 0 1.022-.089 2.808 2.808 0 0 1-.926-.362 2.942 2.942 0 0 1-.728-.633 2.839 2.839 0 0 1-.65-1.822v-.033c.402.227.837.348 1.306.362a2.943 2.943 0 0 1-.936-1.04 2.955 2.955 0 0 1-.253-.649 2.945 2.945 0 0 1 .007-1.453c.063-.243.161-.474.294-.693.364.451.77.856 1.216 1.213a8.215 8.215 0 0 0 3.008 1.525 7.965 7.965 0 0 0 1.695.263 2.15 2.15 0 0 1-.058-.325 3.265 3.265 0 0 1-.017-.331c0-.397.075-.77.226-1.118a2.892 2.892 0 0 1 1.528-1.528 2.79 2.79 0 0 1 1.117-.225 2.846 2.846 0 0 1 2.099.909 5.7 5.7 0 0 0 1.818-.698 2.815 2.815 0 0 1-1.258 1.586A5.704 5.704 0 0 0 15 3.785z"
        /> < title > { title } < / title > < / svg >
    }
}
