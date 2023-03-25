#[cfg(feature = "IoNuclearSharp")]
use leptos::*;
#[cfg(feature = "IoNuclearSharp")]
///This icon requires the feature `IoNuclearSharp` to be enabled.
#[component]
pub fn NuclearSharp(
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
        "http://www.w3.org/2000/svg" > < circle xmlns = "http://www.w3.org/2000/svg" cx =
        "256" cy = "256" r = "48" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M223.47,335.59l-51.71,68a169.73,169.73,0,0,0,168.48,0l-51.71-68" style =
        "fill:none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M403.08,108.92A208,208,0,0,0,108.92,403.08,208,208,0,0,0,403.08,108.92ZM342,256a86.13,86.13,0,0,1-53.47,79.59l51.71,68a169.73,169.73,0,0,1-168.48,0l51.71-68a86,86,0,0,1-50.56-101.77l-85.48.09a170.21,170.21,0,0,1,73.83-119L199.2,191.5a85.78,85.78,0,0,1,113.6,0l37.94-76.59a170.21,170.21,0,0,1,73.83,119l-85.48-.09A85.87,85.87,0,0,1,342,256Z"
        /> < title > { title } < / title > < / svg >
    }
}
