#[cfg(feature = "RiWeatherLineBlaze")]
use leptos::*;
#[cfg(feature = "RiWeatherLineBlaze")]
///This icon requires the feature `RiWeatherLineBlaze` to be enabled.
#[component]
pub fn Blaze(
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
        "M19 9c.667 1.06 1 2.394 1 4 0 3-3.5 4-5 9-.667-.575-1-1.408-1-2.5 0-3.482 5-5.29 5-10.5zm-4.5-4a8.31 8.31 0 0 1 1 4c0 5-6 6-4 13C9.833 20.84 9 19.173 9 17c0-3.325 5.5-6 5.5-12zM10 1c.667 1.333 1 2.833 1 4.5 0 6-9 7.5-3 16.5-2.5-.5-4.5-3-4.5-6C3.5 9.5 10 8.5 10 1z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
