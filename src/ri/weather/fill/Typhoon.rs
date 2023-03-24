#[cfg(feature = "RiWeatherFillTyphoon")]
use leptos::*;
#[cfg(feature = "RiWeatherFillTyphoon")]
///This icon requires the feature `RiWeatherFillTyphoon` to be enabled.
#[component]
pub fn Typhoon(
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
        "none" d = "M0 0h24v24H0z" />< path d =
        "M17.654 1.7l-2.782 2.533a9.137 9.137 0 0 1 3.49 1.973c3.512 3.2 3.512 8.388 0 11.588-2.592 2.36-6.598 3.862-12.016 4.506l2.782-2.533a9.137 9.137 0 0 1-3.49-1.973c-3.512-3.2-3.533-8.369 0-11.588C8.23 3.846 12.237 2.344 17.655 1.7zM12 8c-2.485 0-4.5 1.79-4.5 4s2.015 4 4.5 4 4.5-1.79 4.5-4-2.015-4-4.5-4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
