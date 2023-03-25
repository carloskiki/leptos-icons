#[cfg(feature = "IoColorPaletteOutline")]
use leptos::*;
#[cfg(feature = "IoColorPaletteOutline")]
///This icon requires the feature `IoColorPaletteOutline` to be enabled.
#[component]
pub fn ColorPaletteOutline(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M430.11,347.9c-6.6-6.1-16.3-7.6-24.6-9-11.5-1.9-15.9-4-22.6-10-14.3-12.7-14.3-31.1,0-43.8l30.3-26.9c46.4-41,46.4-108.2,0-149.2-34.2-30.1-80.1-45-127.8-45-55.7,0-113.9,20.3-158.8,60.1-83.5,73.8-83.5,194.7,0,268.5,41.5,36.7,97.5,55,152.9,55.4h1.7c55.4,0,110-17.9,148.8-52.4C444.41,382.9,442,359,430.11,347.9Z"
        style = "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< circle
        xmlns = "http://www.w3.org/2000/svg" cx = "144" cy = "208" r = "32" />< circle
        xmlns = "http://www.w3.org/2000/svg" cx = "152" cy = "311" r = "32" />< circle
        xmlns = "http://www.w3.org/2000/svg" cx = "224" cy = "144" r = "32" />< circle
        xmlns = "http://www.w3.org/2000/svg" cx = "256" cy = "367" r = "48" />< circle
        xmlns = "http://www.w3.org/2000/svg" cx = "328" cy = "144" r = "32" /> < title >
        { title } < / title > < / svg >
    }
}
