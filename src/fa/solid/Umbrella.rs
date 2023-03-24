#[cfg(feature = "FaSolidUmbrella")]
use leptos::*;
#[cfg(feature = "FaSolidUmbrella")]
///This icon requires the feature `FaSolidUmbrella` to be enabled.
#[component]
pub fn Umbrella(
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
        stroke_witdh = "0" style = style viewBox = "0 0 576 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M286 0c17.7 0 32 14.3 32 32V49.7C449.8 63.4 555.7 161 571.9 285.9c2 15.6-17.3 24.4-27.8 12.7C530.1 283 502.8 272 478 272c-38.7 0-71 27.5-78.4 64.1c-1.7 8.7-8.7 15.9-17.6 15.9s-15.8-7.2-17.6-15.9C357 299.5 324.7 272 286 272s-71 27.5-78.4 64.1c-1.7 8.7-8.7 15.9-17.6 15.9s-15.8-7.2-17.6-15.9C165 299.5 132.7 272 94 272c-24.8 0-52.1 11-66.1 26.7C17.4 310.4-1.9 301.5 .1 285.9C16.3 161 122.2 63.4 254 49.7V32c0-17.7 14.3-32 32-32zm0 304c12.3 0 23.5 4.6 32 12.2V430.6c0 45-36.5 81.4-81.4 81.4c-30.8 0-59-17.4-72.8-45l-2.3-4.7c-7.9-15.8-1.5-35 14.3-42.9s35-1.5 42.9 14.3l2.3 4.7c3 5.9 9 9.6 15.6 9.6c9.6 0 17.4-7.8 17.4-17.4V316.2c8.5-7.6 19.7-12.2 32-12.2z"
        /> < title > { title } < / title > < / svg >
    }
}
