#[cfg(feature = "IoWaterSharp")]
use leptos::*;
#[cfg(feature = "IoWaterSharp")]
///This icon requires the feature `IoWaterSharp` to be enabled.
#[component]
pub fn WaterSharp(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,43.91s-144,158.3-144,270.3c0,88.36,55.64,144,144,144s144-55.64,144-144C400,202.21,256,43.91,256,43.91Zm16,362.3v-24a60.07,60.07,0,0,0,60-60h24A84.09,84.09,0,0,1,272,406.21Z"
        /> < title > { title } < / title > < / svg >
    }
}
