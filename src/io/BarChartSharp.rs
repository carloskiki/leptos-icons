#[cfg(feature = "IoBarChartSharp")]
use leptos::*;
#[cfg(feature = "IoBarChartSharp")]
///This icon requires the feature `IoBarChartSharp` to be enabled.
#[component]
pub fn BarChartSharp(
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
        points = "496 496 16 496 16 16 48 16 48 464 496 464 496 496" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M192,432H80V208H192Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M336,432H224V160H336Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M479.64,432h-112V96h112Z" /> < title > { title
        } < / title > < / svg >
    }
}
