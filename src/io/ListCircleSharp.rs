#[cfg(feature = "IoListCircleSharp")]
use leptos::*;
#[cfg(feature = "IoListCircleSharp")]
///This icon requires the feature `IoListCircleSharp` to be enabled.
#[component]
pub fn ListCircleSharp(
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
        "M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48ZM192,335.5a16,16,0,0,1-16,16H160a16,16,0,0,1-16-16v-16a16,16,0,0,1,16-16h16a16,16,0,0,1,16,16Zm0-71a16,16,0,0,1-16,16H160a16,16,0,0,1-16-16v-16a16,16,0,0,1,16-16h16a16,16,0,0,1,16,16Zm0-72a16,16,0,0,1-16,16H160a16,16,0,0,1-16-16v-16a16,16,0,0,1,16-16h16a16,16,0,0,1,16,16Zm176,151H212.67v-32H368Zm0-71H212.67v-32H368Zm0-72H212.67v-32H368Z"
        /> < title > { title } < / title > < / svg >
    }
}
