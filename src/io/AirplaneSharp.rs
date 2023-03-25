#[cfg(feature = "IoAirplaneSharp")]
use leptos::*;
#[cfg(feature = "IoAirplaneSharp")]
///This icon requires the feature `IoAirplaneSharp` to be enabled.
#[component]
pub fn AirplaneSharp(
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
        "M407.72,208c-2.72,0-14.44.08-18.67.31l-57.77,1.52L198.06,48H135.25l74.59,164.61-97.31,1.44L68.25,160H16.14l20.61,94.18c.15.54.33,1.07.53,1.59a.26.26,0,0,1,0,.15,15.42,15.42,0,0,0-.53,1.58L15.86,352H67.64l45.45-55,96.77,2.17L135.24,464h63l133-161.75,57.77,1.54c4.29.23,16,.31,18.66.31,24.35,0,44.27-3.34,59.21-9.94C492.22,283,496,265.46,496,256,496,225.94,463,208,407.72,208Zm-71.29,87.9v0Z"
        /> < title > { title } < / title > < / svg >
    }
}
