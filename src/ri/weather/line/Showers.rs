#[cfg(feature = "RiWeatherLineShowers")]
use leptos::*;
#[cfg(feature = "RiWeatherLineShowers")]
///This icon requires the feature `RiWeatherLineShowers` to be enabled.
#[component]
pub fn Showers(
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
        "M5 16.93a8 8 0 1 1 11.458-9.831A5.5 5.5 0 0 1 19 17.793v-2.13a3.5 3.5 0 1 0-4-5.612V10a6 6 0 1 0-10 4.472v2.458zM7 16h2v4H7v-4zm8 0h2v4h-2v-4zm-4 3h2v4h-2v-4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
