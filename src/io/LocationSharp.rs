#[cfg(feature = "IoLocationSharp")]
use leptos::*;
#[cfg(feature = "IoLocationSharp")]
///This icon requires the feature `IoLocationSharp` to be enabled.
#[component]
pub fn LocationSharp(
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
        "M256,32C167.67,32,96,96.51,96,176c0,128,160,304,160,304S416,304,416,176C416,96.51,344.33,32,256,32Zm0,224a64,64,0,1,1,64-64A64.07,64.07,0,0,1,256,256Z"
        /> < title > { title } < / title > < / svg >
    }
}
