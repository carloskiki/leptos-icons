#[cfg(feature = "IoReloadCircleSharp")]
use leptos::*;
#[cfg(feature = "IoReloadCircleSharp")]
///This icon requires the feature `IoReloadCircleSharp` to be enabled.
#[component]
pub fn ReloadCircleSharp(
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
        "M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48ZM376,238.77H287l36.88-36.88-5.6-6.51a87.38,87.38,0,1,0-62.94,148,87.55,87.55,0,0,0,82.42-58.25L343.13,270l30.17,10.67L368,295.8A119.4,119.4,0,1,1,255.38,136.62a118.34,118.34,0,0,1,86.36,36.95l.56.62,4.31,5L376,149.81Z"
        /> < title > { title } < / title > < / svg >
    }
}
