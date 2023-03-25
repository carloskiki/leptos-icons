#[cfg(feature = "IoFlashOffSharp")]
use leptos::*;
#[cfg(feature = "IoFlashOffSharp")]
///This icon requires the feature `IoFlashOffSharp` to be enabled.
#[component]
pub fn FlashOffSharp(
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
        "240.44" y = "0.03" width = "31.11" height = "511.95" transform =
        "translate(-106.04 256) rotate(-45)" />< polygon xmlns =
        "http://www.w3.org/2000/svg" points =
        "80 304 224 304 192 496 300.18 366.18 151.82 217.82 80 304" />< polygon xmlns =
        "http://www.w3.org/2000/svg" points =
        "432 208 288 208 320 16 211.82 145.82 360.18 294.18 432 208" /> < title > { title
        } < / title > < / svg >
    }
}
