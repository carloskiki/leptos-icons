#[cfg(feature = "IoFilterSharp")]
use leptos::*;
#[cfg(feature = "IoFilterSharp")]
///This icon requires the feature `IoFilterSharp` to be enabled.
#[component]
pub fn FilterSharp(
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
        "16" y = "120" width = "480" height = "48" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "96" y = "232" width = "320" height = "48" /><
        rect xmlns = "http://www.w3.org/2000/svg" x = "192" y = "344" width = "128"
        height = "48" /> < title > { title } < / title > < / svg >
    }
}
