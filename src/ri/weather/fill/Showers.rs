#[cfg(feature = "RiWeatherFillShowers")]
use leptos::*;
#[cfg(feature = "RiWeatherFillShowers")]
///This icon requires the feature `RiWeatherFillShowers` to be enabled.
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
        "M15 18H9v3H7v-3.252a8 8 0 1 1 9.458-10.65A5.5 5.5 0 1 1 17.5 18l-.5.001v3h-2v-3zm-4 2h2v3h-2v-3z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
