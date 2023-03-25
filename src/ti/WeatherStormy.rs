#[cfg(feature = "TiWeatherStormy")]
use leptos::*;
#[cfg(feature = "TiWeatherStormy")]
///This icon requires the feature `TiWeatherStormy` to be enabled.
#[component]
pub fn WeatherStormy(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < g xmlns = "http://www.w3.org/2000/svg" >< path
        d =
        "M17 18c-.553 0-1-.447-1-1s.447-1 1-1c1.654 0 3-1.346 3-3s-1.346-3-3-3c-.238 0-.496.042-.813.131l-1.071.301-.186-1.098c-.326-1.932-1.979-3.334-3.93-3.334-2.205 0-4 1.794-4 4 0 .274.027.545.082.806l.26 1.24-1.436-.052c-1.01.006-1.906.903-1.906 2.006s.896 2 2 2c.553 0 1 .447 1 1s-.447 1-1 1c-2.205 0-4-1.794-4-4 0-1.861 1.277-3.429 3.002-3.874l-.002-.126c0-3.309 2.691-6 6-6 2.587 0 4.824 1.638 5.649 4.015 2.925-.241 5.351 2.112 5.351 4.985 0 2.757-2.243 5-5 5zM12.639 14l-4.5 4.051 3 1.449-1.5 3.5 4.5-4.05-3-1.45z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
