#[cfg(feature = "FaSolidTablets")]
use leptos::*;
#[cfg(feature = "FaSolidTablets")]
///This icon requires the feature `FaSolidTablets` to be enabled.
#[component]
pub fn Tablets(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M614.3 247c16.3-25 25.7-54.9 25.7-87C640 71.6 568.4 0 480 0c-32.1 0-61.9 9.4-87 25.7c-7.9 5.2-8.5 16.2-1.8 22.9L591.4 248.8c6.7 6.7 17.8 6.2 22.9-1.8zM567 294.3c7.9-5.2 8.5-16.2 1.8-22.9L368.6 71.2c-6.7-6.7-17.8-6.2-22.9 1.8c-16.3 25-25.7 54.9-25.7 87c0 88.4 71.6 160 160 160c32.1 0 61.9-9.4 87-25.7zM301.5 368H18.5c-9.5 0-16.9 8.2-15 17.5C18.9 457.8 83.1 512 160 512s141.1-54.2 156.5-126.5c2-9.3-5.5-17.5-15-17.5zm0-32c9.5 0 16.9-8.2 15-17.5C301.1 246.2 236.9 192 160 192S18.9 246.2 3.5 318.5c-2 9.3 5.5 17.5 15 17.5H301.5z"
        /> < title > { title } < / title > < / svg >
    }
}
