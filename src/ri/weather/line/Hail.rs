#[cfg(feature = "RiWeatherLineHail")]
use leptos::*;
#[cfg(feature = "RiWeatherLineHail")]
///This icon requires the feature `RiWeatherLineHail` to be enabled.
#[component]
pub fn Hail(
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
        "M6 17.418A8.003 8.003 0 0 1 9 2a8.003 8.003 0 0 1 7.458 5.099A5.5 5.5 0 0 1 19 17.793v-2.13a3.5 3.5 0 1 0-4-5.612V10a6 6 0 1 0-9 5.197v2.221zM10 17a2 2 0 1 1 0-4 2 2 0 0 1 0 4zm5 3a2 2 0 1 1 0-4 2 2 0 0 1 0 4zm-5 3a2 2 0 1 1 0-4 2 2 0 0 1 0 4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
