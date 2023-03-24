#[cfg(feature = "IoColorWandSharp")]
use leptos::*;
#[cfg(feature = "IoColorWandSharp")]
///This icon requires the feature `IoColorWandSharp` to be enabled.
#[component]
pub fn ColorWandSharp(
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
        "http://www.w3.org/2000/svg" > < rect xmlns = "http://www.w3.org/2000/svg" x =
        "158.6" y = "150.86" width = "95.03" height = "110.51" transform =
        "translate(-85.38 206.12) rotate(-45)" />< polygon xmlns =
        "http://www.w3.org/2000/svg" points =
        "301.41 234.21 234.22 301.41 412 480 480 412 301.41 234.21" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "32" y = "176" width = "80" height = "32" /><
        rect xmlns = "http://www.w3.org/2000/svg" x = "91.22" y = "67.22" width = "32"
        height = "80" transform = "translate(-44.41 107.22) rotate(-45)" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "176" y = "32" width = "32" height = "80" /><
        rect xmlns = "http://www.w3.org/2000/svg" x = "236.92" y = "91.22" width = "80"
        height = "32" transform = "translate(5.29 227.22) rotate(-45)" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "67.22" y = "260.92" width = "80" height = "32"
        transform = "translate(-164.41 156.92) rotate(-45)" /> < title > { title } < /
        title > < / svg >
    }
}
