#[cfg(feature = "IoFlashlightSharp")]
use leptos::*;
#[cfg(feature = "IoFlashlightSharp")]
///This icon requires the feature `IoFlashlightSharp` to be enabled.
#[component]
pub fn FlashlightSharp(
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
        "http://www.w3.org/2000/svg" > < polygon xmlns = "http://www.w3.org/2000/svg"
        points = "330 16 287.32 58.7 453.3 224.68 496 182 330 16" />< ellipse xmlns =
        "http://www.w3.org/2000/svg" cx = "224.68" cy = "287.3" rx = "20.03" ry = "19.96"
        style = "fill:none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M429.21,243.85,268,82.59,249.65,168,16,402l94,94L344.23,262.2Zm-189,56.07a20,20,0,1,1,0-25.25A20,20,0,0,1,240.19,299.92Z"
        /> < title > { title } < / title > < / svg >
    }
}
