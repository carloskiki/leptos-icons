#[cfg(feature = "IoHeartCircle")]
use leptos::*;
#[cfg(feature = "IoHeartCircle")]
///This icon requires the feature `IoHeartCircle` to be enabled.
#[component]
pub fn HeartCircle(
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
        "M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48Zm74.69,252.82c-9.38,11.44-26.4,29.73-65.7,56.41a15.93,15.93,0,0,1-18,0c-39.3-26.68-56.32-45-65.7-56.41-20-24.37-29.58-49.4-29.3-76.5.31-31.06,25.22-56.33,55.53-56.33,20.4,0,35,10.63,44.1,20.41a6,6,0,0,0,8.72,0c9.11-9.78,23.7-20.41,44.1-20.41,30.31,0,55.22,25.27,55.53,56.33C360.27,251.42,350.68,276.45,330.69,300.82Z"
        /> < title > { title } < / title > < / svg >
    }
}
