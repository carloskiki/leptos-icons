#[cfg(feature = "IoShieldSharp")]
use leptos::*;
#[cfg(feature = "IoShieldSharp")]
///This icon requires the feature `IoShieldSharp` to be enabled.
#[component]
pub fn ShieldSharp(
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
        "M256,32C174,69.06,121.38,86.46,32,96c0,77.59,5.27,133.36,25.29,184.51a348.86,348.86,0,0,0,71.43,112.41C178.32,445.58,232.89,473.32,256,480c23.11-6.68,77.68-34.42,127.28-87.08a348.86,348.86,0,0,0,71.43-112.41C474.73,229.36,480,173.59,480,96,390.62,86.46,338,69.06,256,32Z"
        /> < title > { title } < / title > < / svg >
    }
}
