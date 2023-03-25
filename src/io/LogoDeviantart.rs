#[cfg(feature = "IoLogoDeviantart")]
use leptos::*;
#[cfg(feature = "IoLogoDeviantart")]
///This icon requires the feature `IoLogoDeviantart` to be enabled.
#[component]
pub fn LogoDeviantart(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < polygon xmlns =
        "http://www.w3.org/2000/svg" points =
        "408 103.28 408 16 407.97 16 318.69 16 309.79 24.78 267.64 103.26 254.39 112 104 112 104 231.85 186.68 231.85 194.04 240.56 104 408.72 104 496 104.02 496 193.3 496 202.21 487.21 244.35 408.73 257.61 400 408 400 408 280.13 325.32 280.13 317.96 271.38 408 103.28"
        /> < title > { title } < / title > < / svg >
    }
}
