#[cfg(feature = "SiPoetry")]
use leptos::*;
#[cfg(feature = "SiPoetry")]
///This icon requires the feature `SiPoetry` to be enabled.
#[component]
pub fn Poetry(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M21.604 0a19.144 19.144 0 0 1-5.268 13.213L2.396 0l13.583 13.583a19.149 19.149 0 0 1-13.583 5.624V0h19.208Zm-1.911 17.297A24.455 24.455 0 0 1 7.189 24l-4.053-4.053a19.91 19.91 0 0 0 13.37-5.838l3.187 3.188Z"
        /> < title > { title } < / title > < / svg >
    }
}
