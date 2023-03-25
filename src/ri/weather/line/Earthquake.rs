#[cfg(feature = "RiWeatherLineEarthquake")]
use leptos::*;
#[cfg(feature = "RiWeatherLineEarthquake")]
///This icon requires the feature `RiWeatherLineEarthquake` to be enabled.
#[component]
pub fn Earthquake(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M5 21a1 1 0 0 1-.993-.883L4 20v-9H1l10.327-9.388a1 1 0 0 1 1.246-.08l.1.08L23 11h-3v9a1 1 0 0 1-.883.993L19 21H5zm7-17.298L6 9.156V19h4.357l1.393-1.5L8 14l5-3-2.5-2 3-3-.5 3 2.5 2-4 3 3.5 3-1.25 2H18V9.157l-6-5.455z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
