#[cfg(feature = "RiWeatherFillRainbow")]
use leptos::*;
#[cfg(feature = "RiWeatherFillRainbow")]
///This icon requires the feature `RiWeatherFillRainbow` to be enabled.
#[component]
pub fn Rainbow(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M12 4c6.075 0 11 4.925 11 11v5h-3v-5a8 8 0 0 0-7.75-7.996L12 7a8 8 0 0 0-7.996 7.75L4 15v5H1v-5C1 8.925 5.925 4 12 4zm0 4a7 7 0 0 1 7 7v5h-3v-5a4 4 0 0 0-3.8-3.995L12 11a4 4 0 0 0-3.995 3.8L8 15v5H5v-5a7 7 0 0 1 7-7zm0 4a3 3 0 0 1 3 3v5H9v-5a3 3 0 0 1 3-3z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
