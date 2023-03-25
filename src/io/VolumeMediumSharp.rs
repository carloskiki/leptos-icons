#[cfg(feature = "IoVolumeMediumSharp")]
use leptos::*;
#[cfg(feature = "IoVolumeMediumSharp")]
///This icon requires the feature `IoVolumeMediumSharp` to be enabled.
#[component]
pub fn VolumeMediumSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < polygon xmlns = "http://www.w3.org/2000/svg"
        points =
        "157.65 176.1 64 176.1 64 335.9 157.65 335.9 288 440 288 72 157.65 176.1" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M352,320c9.74-19.41,16-40.81,16-64,0-23.51-6-44.4-16-64" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M400,368c19.48-34,32-64,32-112s-12-77.7-32-112" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
