#[cfg(feature = "IoContrastSharp")]
use leptos::*;
#[cfg(feature = "IoContrastSharp")]
///This icon requires the feature `IoContrastSharp` to be enabled.
#[component]
pub fn ContrastSharp(
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
        "M256,32C132.29,32,32,132.29,32,256S132.29,480,256,480,480,379.71,480,256,379.71,32,256,32ZM128.72,383.28A180,180,0,0,1,256,76V436A178.82,178.82,0,0,1,128.72,383.28Z"
        /> < title > { title } < / title > < / svg >
    }
}
