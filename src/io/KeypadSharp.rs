#[cfg(feature = "IoKeypadSharp")]
use leptos::*;
#[cfg(feature = "IoKeypadSharp")]
///This icon requires the feature `IoKeypadSharp` to be enabled.
#[component]
pub fn KeypadSharp(
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
        "http://www.w3.org/2000/svg" > < rect xmlns = "http://www.w3.org/2000/svg" x =
        "80" y = "16" width = "96" height = "96" rx = "8" ry = "8" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "208" y = "16" width = "96" height = "96" rx =
        "8" ry = "8" />< rect xmlns = "http://www.w3.org/2000/svg" x = "336" y = "16"
        width = "96" height = "96" rx = "8" ry = "8" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "80" y = "144" width = "96" height = "96" rx =
        "8" ry = "8" />< rect xmlns = "http://www.w3.org/2000/svg" x = "208" y = "144"
        width = "96" height = "96" rx = "8" ry = "8" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "336" y = "144" width = "96" height = "96" rx =
        "8" ry = "8" />< rect xmlns = "http://www.w3.org/2000/svg" x = "80" y = "272"
        width = "96" height = "96" rx = "8" ry = "8" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "208" y = "272" width = "96" height = "96" rx =
        "8" ry = "8" />< rect xmlns = "http://www.w3.org/2000/svg" x = "208" y = "400"
        width = "96" height = "96" rx = "8" ry = "8" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "336" y = "272" width = "96" height = "96" rx =
        "8" ry = "8" /> < title > { title } < / title > < / svg >
    }
}
