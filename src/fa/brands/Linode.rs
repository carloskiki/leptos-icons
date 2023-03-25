#[cfg(feature = "FaBrandsLinode")]
use leptos::*;
#[cfg(feature = "FaBrandsLinode")]
///This icon requires the feature `FaBrandsLinode` to be enabled.
#[component]
pub fn Linode(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M366.036,186.867l-59.5,36.871-.838,36.871-29.329-19.273-39.384,24.3c2.238,55.211,2.483,59.271,2.51,59.5l-97.2,65.359L127.214,285.748l108.1-62.01L195.09,197.761l-75.417,38.547L98.723,93.015,227.771,43.574,136.432,0,10.737,39.385,38.39,174.3l41.9,32.681L48.445,222.062,69.394,323.457,98.723,351.11,77.774,363.679l16.76,78.769L160.733,512c-10.8-74.842-11.658-78.641-11.725-78.773l77.925-55.3c16.759-12.57,15.083-10.894,15.083-10.894l.838,24.3,33.519,28.491-.838-77.093,46.927-33.519,26.815-18.435-2.514,36.033,25.139,17.6,6.7-74.579,58.657-43.575Z"
        /> < title > { title } < / title > < / svg >
    }
}
