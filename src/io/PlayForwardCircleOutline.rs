#[cfg(feature = "IoPlayForwardCircleOutline")]
use leptos::*;
#[cfg(feature = "IoPlayForwardCircleOutline")]
///This icon requires the feature `IoPlayForwardCircleOutline` to be enabled.
#[component]
pub fn PlayForwardCircleOutline(
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
        "M448,256c0-106-86-192-192-192S64,150,64,256s86,192,192,192S448,362,448,256Z"
        style = "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M394.77,246.7,280.32,177.56A10.78,10.78,0,0,0,264,186.87v53.32L160.32,177.56A10.78,10.78,0,0,0,144,186.87V325.13a10.78,10.78,0,0,0,16.32,9.31L264,271.81v53.32a10.78,10.78,0,0,0,16.32,9.31L394.77,265.3A10.89,10.89,0,0,0,394.77,246.7Z"
        /> < title > { title } < / title > < / svg >
    }
}
