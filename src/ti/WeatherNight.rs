#[cfg(feature = "TiWeatherNight")]
use leptos::*;
#[cfg(feature = "TiWeatherNight")]
///This icon requires the feature `TiWeatherNight` to be enabled.
#[component]
pub fn WeatherNight(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.5 20c-.861 0-1.71-.151-2.523-.451l-1.317-.485.89-1.087c1.275-1.56 1.95-3.454 1.95-5.477s-.675-3.917-1.951-5.477l-.89-1.087 1.317-.485c.814-.3 1.663-.451 2.524-.451 4.136 0 7.5 3.364 7.5 7.5s-3.364 7.5-7.5 7.5zm-.509-2.024c.169.016.339.024.509.024 3.032 0 5.5-2.468 5.5-5.5s-2.468-5.5-5.5-5.5c-.17 0-.34.008-.509.024.991 1.645 1.509 3.511 1.509 5.476s-.518 3.831-1.509 5.476z"
        /> < title > { title } < / title > < / svg >
    }
}
