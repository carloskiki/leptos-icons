#[cfg(feature = "TiWeatherShower")]
use leptos::*;
#[cfg(feature = "TiWeatherShower")]
///This icon requires the feature `TiWeatherShower` to be enabled.
#[component]
pub fn WeatherShower(
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
        = "http://www.w3.org/2000/svg" > < g xmlns = "http://www.w3.org/2000/svg" >< path
        d =
        "M17 18c-.552 0-1-.447-1-1s.448-1 1-1c1.654 0 3-1.346 3-3s-1.346-3-3-3c-.243 0-.5.041-.81.13l-1.075.307-.185-1.103c-.326-1.932-1.978-3.334-3.93-3.334-2.206 0-4 1.794-4 4 0 .272.027.545.082.811l.244 1.199-1.42-.016c-1.009.006-1.906.903-1.906 2.006s.897 2 2 2c.552 0 1 .447 1 1s-.448 1-1 1c-2.206 0-4-1.794-4-4 0-1.861 1.277-3.429 3.001-3.874l-.001-.126c0-3.309 2.691-6 6-6 2.587 0 4.824 1.638 5.65 4.015 2.939-.244 5.35 2.113 5.35 4.985 0 2.757-2.243 5-5 5zM10.5 18l1-3 1 3c.184.553-.114 1.149-.667 1.333-.552.185-1.149-.114-1.333-.666-.075-.226-.07-.458 0-.667zM13.5 20l1-3 1 3c.184.553-.114 1.149-.667 1.333-.552.185-1.149-.114-1.333-.666-.075-.226-.07-.458 0-.667zM7.5 20l1-3 1 3c.184.553-.114 1.149-.667 1.333-.552.185-1.149-.114-1.333-.666-.075-.226-.07-.458 0-.667z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
