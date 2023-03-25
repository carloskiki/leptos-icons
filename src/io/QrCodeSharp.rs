#[cfg(feature = "IoQrCodeSharp")]
use leptos::*;
#[cfg(feature = "IoQrCodeSharp")]
///This icon requires the feature `IoQrCodeSharp` to be enabled.
#[component]
pub fn QrCodeSharp(
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
        "336" y = "336" width = "80" height = "80" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "272" y = "272" width = "64" height = "64" /><
        rect xmlns = "http://www.w3.org/2000/svg" x = "416" y = "416" width = "64" height
        = "64" />< rect xmlns = "http://www.w3.org/2000/svg" x = "432" y = "272" width =
        "48" height = "48" />< rect xmlns = "http://www.w3.org/2000/svg" x = "272" y =
        "432" width = "48" height = "48" />< rect xmlns = "http://www.w3.org/2000/svg" x
        = "336" y = "96" width = "80" height = "80" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M480,240H272V32H480ZM316,196H436V76H316Z" /><
        rect xmlns = "http://www.w3.org/2000/svg" x = "96" y = "96" width = "80" height =
        "80" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M240,240H32V32H240ZM76,196H196V76H76Z" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "96" y = "336" width = "80" height = "80" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M240,480H32V272H240ZM76,436H196V316H76Z" /> < title > { title } < / title > < /
        svg >
    }
}
